```md
# Copilot Instructions – Galactus Market Intelligence Platform

## Mission
Build a comprehensive, auditable, and research-grade market data platform by capturing  
all analytically valuable data published by NSE, using open-source tooling,  
and storing it in a lakehouse architecture optimized for financial research.

---

## 1. Project Scope & Intent (READ FIRST)

This repository implements a **full-coverage NSE market intelligence platform**, not just a bhavcopy pipeline.

The system:

- Scrapes NSE data using **nsemine** and **Playwright**
- Captures **structured data**, **semi-structured data**, and **PDFs**
- Separates **daily incremental jobs** and **historical correction/backfill jobs**
- Stores **raw, immutable data** and **clean, canonical datasets**
- Uses **Apache Hudi** as the system of record
- Registers metadata via **Hive Metastore**
- Syncs curated datasets to **ClickHouse** for low-latency analytics
- Uses **Apache Airflow** for orchestration, retries, and observability

This is a **financial research system**.

> Correctness, determinism, auditability, and reproducibility are more important than speed.

---

## 2. Core Architectural Laws (NON-NEGOTIABLE)

### 2.1 Layered Data Architecture (STRICT)

```

Scraping → Bronze → Silver (Hudi) → Gold (Research / ClickHouse)

```

### Scraping Layer (nsemine / Playwright)

**Purpose:** Extract data exactly as published by NSE.

**Allowed:**
- HTTP requests
- Browser automation
- File downloads (CSV, XLS, PDF)

**Forbidden:**
- Data cleaning
- Column renaming
- Deduplication
- Schema enforcement

> Scrapers must behave like **dumb recorders**, not data engineers.

---

### Bronze Layer (Raw Storage)

- Stores **exact copies** of NSE outputs
- Includes:
  - CSV
  - XLS/XLSX
  - JSON
  - PDF
- **Immutable**
- Date-partitioned by **download date**

> Bronze is **evidence**, not analytics.

---

### Silver Layer (Canonical – Apache Hudi)

- Cleaned, validated, structured data
- Enforced schemas
- Primary keys & partitioning
- Point-in-time correctness

> Silver is the **source of truth**.

---

### Gold Layer (Derived / Research)

- Aggregations
- Factors
- Feature tables
- Backtest-ready datasets
- ClickHouse syncs

> Gold can be rebuilt. **Silver cannot.**

---

## 3. Scraping Rules (nsemine & Playwright)

### 3.1 Tooling Guidance

- Prefer **nsemine** where APIs are stable and available
- Use **Playwright** when:
  - NSE blocks direct HTTP
  - Data is rendered dynamically
  - PDFs are gated behind JS flows

🚫 Copilot must **not** introduce Selenium or custom browsers.

---

### 3.2 Scraping Discipline

- Respect NSE rate limits
- Use randomized delays
- Handle:
  - Session expiry
  - CAPTCHA failure
  - Partial downloads

> Scraping failures must be **explicit and observable**.  
> Silent failure is unacceptable.

---

### 3.3 Output Rules

Each scrape writes to:

```

data/bronze/<dataset>/<source_date>/<file>

```

Filenames must preserve:
- NSE naming
- Report dates
- Version identifiers (if present)

🚫 Never overwrite Bronze data.

---

## 4. Daily Jobs vs Historical Jobs (VERY IMPORTANT)

### 4.1 Daily Jobs

**Purpose:**
- Capture today’s NSE state
- Minimal latency
- Incremental

**Rules:**
- One trade date per run
- Fail fast if expected data is missing
- No date loops inside daily jobs

---

### 4.2 Historical Jobs

**Purpose:**
- Backfill
- Repair
- Correct missing or corrupt data

**Rules:**
- Explicit `start_date` and `end_date`
- Deterministic re-runs
- Partition-scoped rewrites only

> Historical jobs must **never** behave like daily jobs.

---

## 5. Apache Hudi Rules (System of Record)

### 5.1 Table Classification

| Dataset Type              | Hudi Table        |
|---------------------------|-------------------|
| Bhavcopy / Prices         | COPY_ON_WRITE     |
| Indices                   | COPY_ON_WRITE     |
| F&O / OI                  | MERGE_ON_READ     |
| Fundamentals              | MERGE_ON_READ     |
| Corporate actions         | MERGE_ON_READ     |
| Announcements metadata    | MERGE_ON_READ     |

---

### 5.2 Mandatory Hudi Options

Every write **MUST** explicitly define:

- `hoodie.table.name`
- `hoodie.datasource.write.recordkey.field`
- `hoodie.datasource.write.precombine.field`
- `hoodie.datasource.write.partitionpath.field`
- `hoodie.datasource.write.operation`
- `hoodie.datasource.write.table.type`

🚫 Defaults are forbidden.

---

### 5.3 Partitioning Rules

**Partition by:**
- `trade_date`
- `announcement_date`
- `financial_period_end`

🚫 **NEVER** partition by:
- `symbol`
- `ISIN`
- `index name`

---

## 6. PDF Storage & Handling (Explicit Requirement)

### 6.1 What PDFs to Store

- Corporate announcements
- Financial result PDFs
- Circulars
- Corporate actions notices
- Regulatory communications

> If NSE publishes a PDF that contains new information, store it.

---

### 6.2 How PDFs Are Stored

#### Bronze
- Raw PDF files
- Immutable
- Stored exactly as downloaded

#### Silver (Hudi – Metadata Only)

Store:
- PDF URI / path
- Symbol(s)
- Announcement date
- Category
- Hash (SHA-256)
- Ingestion timestamp

🚫 Do **NOT** embed binary PDFs inside Hudi tables.

---

### 6.3 Optional Gold Enhancements

- OCR / text extraction
- NLP embeddings
- Event classification

> These are **derived**, never canonical.

---

## 7. Spark Processing Rules

### 7.1 APIs
- Use **Spark DataFrame / SQL APIs only**
- 🚫 RDDs are prohibited

---

### 7.2 Schema Discipline

- Explicit schemas only
- Financial types must use:
  - `DECIMAL`
  - `DATE`
  - `TIMESTAMP`

🚫 Never use `FLOAT` / `DOUBLE` for prices unless explicitly justified.

---

### 7.3 Data Quality Checks (Mandatory)

- Price sanity
- Null checks on keys
- Duplicate detection
- Referential integrity (symbol master)

> Invalid data must be **quarantined**.

---

## 8. Airflow Rules

### 8.1 DAG Design

Separate DAGs for:
- Scraping
- Daily processing
- Historical backfill

> DAGs orchestrate only.  
> No business logic in DAG files.

---

### 8.2 Failure Semantics

- Exponential backoff retries
- Trading calendar awareness
- Missing NSE data must fail loudly

---

## 9. ClickHouse Sync Rules

- ClickHouse is a **serving layer**
- Data must originate from **Silver (Hudi)**

Syncs must be:
- Incremental
- Idempotent
- Partition-aware

🚫 No direct scraping → ClickHouse paths allowed.

---

## 10. Reprocessing & Corrections

| Scenario             | Strategy                         |
|----------------------|----------------------------------|
| Same-day rerun       | Hudi upsert                      |
| Partial scrape       | Re-scrape + upsert               |
| Historical fix       | `insert_overwrite` (partition)   |
| Schema evolution     | Additive only                    |

---

## 11. Logging, Lineage & Auditability

Log at minimum:
- Dataset name
- Date range
- Input record count
- Output record count
- Hudi commit instant

🚫 Never log:
- Full datasets
- Raw PDFs
- Credentials

---

## 12. Code Quality & Research Discipline

- Prefer clarity over cleverness
- No hardcoded paths, dates, or credentials
- Python requires type hints
- Functions must be small, composable, and testable

> Copilot must generate **production-grade**, not tutorial code.

---

## 13. What Copilot MUST NOT Do

- Mix scraping and transformation logic
- Drop or overwrite Bronze data
- Bypass Hudi
- Invent schemas, keys, or partitions
- Introduce Pandas for large datasets
- “Fix” data silently

---

## 14. Optimization Philosophy (Research-Driven)

1. Correctness  
2. Point-in-time accuracy  
3. Auditability  
4. Determinism  
5. Performance  
6. Convenience  

> In financial research, **wrong data is worse than slow data**.

---

## 15. Open-Source Optimization Preferences

When optimizing, prefer:
- Apache Hudi metadata & clustering
- Spark AQE
- Partition pruning
- Incremental processing
- Open, battle-tested libraries

🚫 Avoid bespoke solutions unless strictly necessary.
```
