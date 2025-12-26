import os
import tempfile
import pytest

from utils.financial_download import FinancialDownloader
from utils.financial_parser import parse_pdf


def network_available() -> bool:
    import requests

    try:
        r = requests.get("https://www.nseindia.com/", timeout=5)
        return r.status_code == 200
    except Exception:
        return False


@pytest.mark.skipif(not network_available(), reason="NSE not reachable from test environment")
def test_download_and_parse_first_announcement():
    dl = FinancialDownloader()
    dl.bootstrap_session(do_get=True)
    # Query a recent short range and pick first announcement
    anns = dl.fetch_announcements("RELIANCE", "01-12-2025", "25-12-2025")
    assert isinstance(anns, list)
    if not anns:
        pytest.skip("No announcements returned for symbol in date range")

    first = anns[0]
    url = first.get("document_url") or first.get("raw", {}).get("documentLink")
    if not url:
        pytest.skip("Announcement had no document URL")

    fd, path = tempfile.mkstemp(suffix=os.path.splitext(url)[-1] or ".pdf")
    os.close(fd)
    try:
        dl.download_document(url, path)
        parsed = parse_pdf(path)
        assert "raw_text" in parsed and parsed["raw_text"].strip() != ""
    finally:
        try:
            os.remove(path)
        except Exception:
            pass
