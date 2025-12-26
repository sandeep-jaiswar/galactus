"""Parsers for financial reports (PDF/XLSX/XBRL) with a normalization helper.

These are lightweight stubs to be implemented incrementally. The `normalize`
function converts a parsed dictionary into the canonical record we plan to
store in Hudi/ClickHouse.
"""
from typing import Dict, Any
from datetime import datetime, date
import xml.etree.ElementTree as ET


def parse_pdf(path: str) -> Dict[str, Any]:
    """Parse a PDF financial report and return a best-effort dict of fields.

    This implementation uses `pdfplumber` to extract text. It returns a dict
    with a `raw_text` entry and tries minimal heuristics to populate
    `company_name` and `report_date` if they appear in ISO-like patterns.
    """
    try:
        import pdfplumber
    except Exception:
        raise RuntimeError("pdfplumber is required for parse_pdf; install it from requirements.txt")

    raw_text = []
    with pdfplumber.open(path) as pdf:
        for page in pdf.pages:
            txt = page.extract_text() or ""
            raw_text.append(txt)

    joined = "\n".join(raw_text)

    # Basic heuristics
    company_name = None
    report_date = None
    # Try first non-empty line as company name
    for line in joined.splitlines():
        s = line.strip()
        if s:
            company_name = s
            break

    # Find ISO-like date in text
    import re

    date_match = re.search(r"(\d{4}-\d{2}-\d{2})", joined)
    if date_match:
        report_date = date_match.group(1)

    return {
        "company_name": company_name,
        "symbol": None,
        "report_date": report_date,
        "report_type": None,
        "period_end": None,
        "metrics": {},
        "raw_text": joined,
    }


def parse_xlsx(path: str) -> Dict[str, Any]:
    """Parse an XLSX/CSV financial report and return extracted fields."""
    return {
        "company_name": None,
        "symbol": None,
        "report_date": None,
        "report_type": None,
        "period_end": None,
        "metrics": {},
    }


def parse_xbrl(path: str) -> Dict[str, Any]:
    """Parse XBRL content or file path and return a normalized record.

    The function accepts either a path to an XBRL XML file or raw XML bytes/string.
    It extracts common fields (symbol, company_name, report_date, period_end)
    and a `metrics` dict with numeric values parsed where possible.
    """
    # Allow passing bytes or str content or file path
    content = None
    if isinstance(path, (bytes, bytearray)):
        content = path
    else:
        try:
            # if looks like XML
            if isinstance(path, str) and path.strip().startswith("<"):
                content = path.encode("utf-8")
            else:
                with open(path, "rb") as fh:
                    content = fh.read()
        except Exception:
            raise RuntimeError("Unable to read XBRL content from path or input")

    try:
        root = ET.fromstring(content)
    except Exception as e:
        raise RuntimeError(f"Invalid XML/XBRL content: {e}") from e

    # Helper to find element text with or without namespace
    def _find_text(r, name, nsmap=None):
        # try namespaced search first
        if nsmap and 'in-bse-fin' in nsmap:
            try:
                elem = r.find(f".//{{{nsmap['in-bse-fin']}}}{name}")
                if elem is not None and elem.text and elem.text.strip():
                    return elem.text.strip()
            except Exception:
                pass
        # fallback - try without namespace
        try:
            elem = r.find(f".//{name}")
            if elem is not None and elem.text and elem.text.strip():
                return elem.text.strip()
        except Exception:
            pass
        return None

    # Try to discover namespace used in this document
    nsmap = {}
    for k, v in root.attrib.items():
        # attributes can include xmlns declarations in some trees
        if k.startswith("{http://www.w3.org/2000/xmlns/}"):
            prefix = k.split("}")[1]
            nsmap[prefix] = v

    # Common fields
    symbol = (_find_text(root, "Symbol", nsmap) or _find_text(root, "ScripCode", nsmap))
    company_name = _find_text(root, "NameOfTheCompany", nsmap) or _find_text(root, "CompanyName", nsmap)
    report_date = _find_text(root, "DateOfBoardMeetingWhenFinancialResultsWereApproved", nsmap) or _find_text(root, "filingDate", nsmap)
    period_end = _find_text(root, "DateOfEndOfReportingPeriod", nsmap) or _find_text(root, "DateOfEndOfFinancialYear", nsmap)
    report_type = _find_text(root, "NatureOfReportStandaloneConsolidated", nsmap) or _find_text(root, "ReportingQuarter", nsmap)

    # Metrics: attempt to extract numeric facts from the XBRL by scanning known keys and any numeric tags
    metrics = {}
    # Known numeric keys to prioritize
    known_numeric = [
        "RevenueFromOperations", "OtherIncome", "Income", "EmployeeBenefitExpense",
        "FinanceCosts", "DepreciationDepletionAndAmortisationExpense", "OtherExpenses",
        "Expenses", "ProfitBeforeExceptionalItemsAndTax", "ProfitBeforeTax", "CurrentTax",
        "DeferredTax", "ProfitLossForPeriod", "PaidUpValueOfEquityShareCapital", "FaceValueOfEquityShareCapital",
        "BasicEarningsLossPerShareFromContinuingOperations", "DilutedEarningsLossPerShareFromContinuingOperations"
    ]

    for key in known_numeric:
        txt = _find_text(root, key, nsmap)
        if txt is not None:
            try:
                metrics[key] = float(txt.replace(',', ''))
            except Exception:
                metrics[key] = txt

    # Additionally, scan all child elements for numeric-looking content and add to metrics if not already present
    for elem in root.iter():
        tag = elem.tag
        # strip namespace
        if '}' in tag:
            tag = tag.split('}', 1)[1]
        if tag in metrics:
            continue
        txt = (elem.text or '').strip() if elem.text else ''
        if not txt:
            continue
        # heuristic: looks like a number
        if any(c.isdigit() for c in txt) and (txt.replace(',', '').replace('.', '').lstrip('-').isdigit()):
            try:
                metrics[tag] = float(txt.replace(',', ''))
            except Exception:
                metrics[tag] = txt

    # Try to coerce dates where possible (leave as strings if parsing fails)
    def _to_iso(dtxt):
        if not dtxt:
            return None
        for fmt in ("%d-%m-%Y", "%Y-%m-%d", "%d-%b-%Y", "%d %b %Y", "%d-%B-%Y"):
            try:
                return datetime.strptime(dtxt[:10], fmt).date().isoformat()
            except Exception:
                continue
        return dtxt

    record = {
        "symbol": symbol,
        "company_name": company_name,
        "report_date": _to_iso(report_date),
        "report_type": report_type,
        "period_end": _to_iso(period_end),
        "metrics": metrics,
        "raw": None,
    }
    return record


def normalize(parsed: Dict[str, Any]) -> Dict[str, Any]:
    """Normalize parsed output into canonical record for Hudi/ClickHouse.

    Expected canonical keys: symbol, company_name, report_type, period_end (date),
    report_date (date), fiscal_year, fiscal_quarter, metrics (dict), raw_file_path, raw_file_format.
    """
    record = {}
    record["symbol"] = parsed.get("symbol")
    record["company_name"] = parsed.get("company_name")
    # Try to coerce date-like fields
    def _to_date(v):
        if v is None:
            return None
        if isinstance(v, date):
            return v
        if isinstance(v, datetime):
            return v.date()
        try:
            return datetime.fromisoformat(str(v)).date()
        except Exception:
            return None

    record["period_end"] = _to_date(parsed.get("period_end"))
    record["report_date"] = _to_date(parsed.get("report_date"))
    record["report_type"] = parsed.get("report_type")
    record["metrics"] = parsed.get("metrics", {}) or {}
    record["fiscal_year"] = None
    record["fiscal_quarter"] = None
    # Attempt basic fiscal year extraction
    if record["period_end"]:
        record["fiscal_year"] = record["period_end"].year

    return record


__all__ = ["parse_pdf", "parse_xlsx", "parse_xbrl", "normalize"]
