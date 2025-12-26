from utils.financial_parser import normalize
from datetime import date


def test_normalize_basic():
    parsed = {
        "symbol": "TEST",
        "company_name": "Test Corp",
        "report_date": "2025-12-20",
        "period_end": "2025-09-30",
        "report_type": "quarterly",
        "metrics": {"total_revenue": 12345.67, "net_profit": 2345.67},
    }

    rec = normalize(parsed)
    assert rec["symbol"] == "TEST"
    assert rec["company_name"] == "Test Corp"
    assert rec["report_type"] == "quarterly"
    assert rec["fiscal_year"] == 2025
    assert isinstance(rec["period_end"], date)
