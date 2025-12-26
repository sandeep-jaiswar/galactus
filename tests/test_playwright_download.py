import os
import sys
from utils.financial_download import PlaywrightFinancialDownloader

out = "/tmp/test_playwright_download.pdf"
url = "https://www.w3.org/WAI/ER/tests/xhtml/testfiles/resources/pdf/dummy.pdf"

print("Starting Playwright dry-run test")

dl = PlaywrightFinancialDownloader(headless=True)
try:
    dl.bootstrap_with_playwright()
    print("Bootstrapped session via Playwright")
    dl.download_document_playwright(url, out)
    size = os.path.getsize(out)
    print("DOWNLOAD_OK", out, size)
    sys.exit(0)
except Exception as e:
    print("DOWNLOAD_ERROR", str(e))
    sys.exit(2)
