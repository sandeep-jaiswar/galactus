import json
from types import SimpleNamespace
from utils.financial_download import FinancialDownloader


class DummyResponse:
    def __init__(self, json_obj, status_code=200):
        self._json = json_obj
        self.status_code = status_code

    def json(self):
        return self._json

    def raise_for_status(self):
        if self.status_code >= 400:
            # emulate requests.HTTPError with response attr
            e = Exception("HTTP %s" % self.status_code)
            e.response = SimpleNamespace(status_code=self.status_code)
            raise e


class DummySession:
    def __init__(self, resp):
        self._resp = resp
        self.headers = {}

    def get(self, url, params=None, timeout=None, stream=False):
        return self._resp


def test_fetch_announcements_basic():
    # Build a dummy JSON similar to what NSE might return
    sample = {"data": [{"symbol": "TEST", "documentLink": "https://nse.test/doc.pdf", "announcementDate": "2025-12-20"}]}
    resp = DummyResponse(sample, status_code=200)
    sess = DummySession(resp)
    dl = FinancialDownloader(session=sess)
    out = dl.fetch_announcements("TEST", "01-12-2025", "25-12-2025")
    assert isinstance(out, list)
    assert out[0]["symbol"] == "TEST"
    assert out[0]["document_url"].endswith("doc.pdf")
