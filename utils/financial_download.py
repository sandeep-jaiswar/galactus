"""Session-aware downloader for NSE announcements and documents.

This module provides a light-weight `FinancialDownloader` that bootstraps
an HTTP session with browser-like headers. Actual announcement parsing
and document URL extraction should be implemented in `fetch_announcements`.
"""
from typing import List, Dict, Optional
import requests

try:
    # Import lazily where available; Playwright is optional dependency
    from playwright.sync_api import sync_playwright
except Exception:  # pragma: no cover - optional dependency may be missing in test env
    sync_playwright = None


class FinancialDownloader:
    def __init__(self, user_agent: Optional[str] = None, session: Optional[requests.Session] = None):
        self.session = session or requests.Session()
        self.user_agent = user_agent or (
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 "
            "(KHTML, like Gecko) Chrome/115.0 Safari/537.36"
        )
        self._set_default_headers()

    def _set_default_headers(self) -> None:
        self.session.headers.update({
            "User-Agent": self.user_agent,
            "Accept": "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8",
            "Accept-Language": "en-US,en;q=0.5",
            "Referer": "https://www.nseindia.com/",
            "Connection": "keep-alive",
        })
    def bootstrap_session(self, do_get: bool = False, timeout: int = 10, max_retries: int = 3) -> requests.Session:
        """Perform a robust bootstrap of an NSE session.

        When `do_get=True` this will attempt to visit the NSE homepage and a
        secondary page to pick up cookies and session state. Retries with
        exponential backoff are performed on transient errors.
        """
        # Ensure headers are browser-like and include Accept
        self.session.headers.update({
            "Accept": "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8",
            "Accept-Language": "en-US,en;q=0.5",
            "Connection": "keep-alive",
        })

        if not do_get:
            return self.session

        import time
        from requests.exceptions import RequestException

        home_url = "https://www.nseindia.com/"
        secondary_paths = [
            "/market-data",
            "/market-data/live-equity-derivatives",
        ]

        attempt = 0
        backoff = 1.0
        while attempt < max_retries:
            try:
                # Visit homepage
                r = self.session.get(home_url, timeout=timeout)
                # Some servers respond with 403; still attempt to fetch secondary
                # pages only if homepage returned 2xx
                if 200 <= r.status_code < 400:
                    # small pause to let cookies settle
                    time.sleep(0.5)
                    for p in secondary_paths:
                        try:
                            self.session.get(home_url.rstrip("/") + p, timeout=timeout)
                            time.sleep(0.2)
                        except RequestException:
                            # ignore individual secondary fetch failures
                            pass
                    return self.session
                else:
                    # Non-2xx homepage response — sleep and retry
                    time.sleep(backoff)
                    attempt += 1
                    backoff *= 2
                    continue
            except RequestException:
                time.sleep(backoff)
                attempt += 1
                backoff *= 2
                continue

        # If we get here, bootstrap failed; return session anyway (calls will detect blocking)
        return self.session

    def fetch_announcements(self, symbol: str, date_from: str, date_to: str, max_retries: int = 3) -> List[Dict]:
        """Placeholder: query NSE corporate announcements API and return a list of announcement metadata.

        Implementers should use `bootstrap_session(do_get=True)` before calling this in production.
        Returned dicts should include at minimum: `symbol`, `report_date`, `document_url`, `report_type`.
        """
        # Attempt to call the commonly-used NSE announcements API. The endpoint and
        # the returned JSON shape may change — return a normalized list of dicts
        # with at least `symbol`, `report_date` and `document_url` when possible.
        api_url = (
            "https://www.nseindia.com/api/corporate-announcements"
        )
        params = {
            "symbol": symbol,
            "from": date_from,  # expected format dd-mm-YYYY by NSE API
            "to": date_to,
            "category": "results",
        }

        import time
        from requests.exceptions import HTTPError, RequestException

        retries = 0
        backoff = 1.0
        while True:
            try:
                resp = self.session.get(api_url, params=params, timeout=15)
                resp.raise_for_status()
                try:
                    data = resp.json()
                except Exception:
                    # Non-JSON response (NSE may return HTML/error pages). Treat as no announcements.
                    return []
                break
            except HTTPError as e:
                status = getattr(e.response, "status_code", None)
                # Retry on 429/503 and transient server errors
                if status in (429, 503, 502, 504) and retries < max_retries:
                    time.sleep(backoff)
                    retries += 1
                    backoff *= 2
                    continue
                raise
            except RequestException:
                if retries < max_retries:
                    time.sleep(backoff)
                    retries += 1
                    backoff *= 2
                    continue
                raise

        # Normalize depending on shape. Common patterns: top-level list or dict with 'data' key.
        items = []
        if isinstance(data, dict):
            # Find likely array candidates
            for key in ("data", "announcements", "results", "rows"):
                if key in data and isinstance(data[key], list):
                    items = data[key]
                    break
            if not items:
                # fallback: if dict itself contains fields of a single announcement
                # return it wrapped
                if any(k in data for k in ("symbol", "doc_url", "documentLink", "announcementDate")):
                    items = [data]
        elif isinstance(data, list):
            items = data

        out: List[Dict] = []
        for it in items:
            # Defensive extraction of fields
            try:
                doc_url = it.get("documentLink") or it.get("doc_url") or it.get("document_url") or it.get("url")
            except Exception:
                doc_url = None

            report_date = None
            for dkey in ("announcementDate", "date", "report_date", "publishedDate"):
                if isinstance(it.get(dkey), str):
                    report_date = it.get(dkey)
                    break

            out.append({
                "symbol": it.get("symbol") or symbol,
                "report_date": report_date,
                "document_url": doc_url,
                "title": it.get("title") or it.get("announcementTitle"),
                "raw": it,
            })

        # If the API supports paging and returns more data, callers can iterate by date range.
        return out

        return out

    def download_document(self, url: str, out_path: str, timeout: int = 30) -> None:
        """Stream-download `url` to `out_path` using the session."""
        with self.session.get(url, stream=True, timeout=timeout) as r:
            r.raise_for_status()
            with open(out_path, "wb") as fh:
                for chunk in r.iter_content(chunk_size=8192):
                    if chunk:
                        fh.write(chunk)


class PlaywrightFinancialDownloader(FinancialDownloader):
    """Downloader that can bootstrap session and download via Playwright when needed.

    This class provides helpers to (a) populate the underlying `requests.Session`
    with cookies obtained from a real browser session and (b) download documents
    using Playwright when site behavior requires a browser.
    """
    def __init__(self, *args, playwright_browser: str = "chromium", headless: bool = True, **kwargs):
        super().__init__(*args, **kwargs)
        self.playwright_browser = playwright_browser
        self.headless = headless

    def bootstrap_with_playwright(self, navigate: bool = True, timeout: int = 30) -> requests.Session:
        """Launch a Playwright browser, visit NSE pages to obtain cookies, and copy them to `self.session`.

        Returns the updated `requests.Session`.
        """
        if sync_playwright is None:
            raise RuntimeError("playwright package is not installed; install with `pip install playwright`")

        with sync_playwright() as p:
            browser_launcher = getattr(p, self.playwright_browser)
            browser = browser_launcher.launch(headless=self.headless)
            context = browser.new_context(user_agent=self.user_agent)
            page = context.new_page()
            try:
                if navigate:
                    home_url = "https://www.nseindia.com/"
                    page.goto(home_url, timeout=timeout * 1000)
                    # visit a couple secondary pages to make cookies settle
                    for pth in ("/market-data", "/market-data/live-equity-derivatives"):
                        try:
                            page.goto(home_url.rstrip("/") + pth, timeout=timeout * 1000)
                        except Exception:
                            pass

                # Extract cookies from context and set into requests session
                cookie_jar = requests.cookies.RequestsCookieJar()
                for c in context.cookies():
                    # Playwright cookie keys: name, value, domain, path, expires, httpOnly, secure
                    cookie_jar.set(c.get("name"), c.get("value"), domain=c.get("domain"), path=c.get("path"))

                # Merge with existing session cookies; prefer browser cookies
                for ck in cookie_jar:
                    self.session.cookies.set_cookie(ck)

                # Also copy any extra headers that Playwright used (e.g., referer) by leaving session headers as-is
            finally:
                try:
                    browser.close()
                except Exception:
                    pass

        return self.session

    def download_document_playwright(self, url: str, out_path: str, timeout: int = 60) -> None:
        """Download a document using Playwright and write it to `out_path`.

        This is useful when direct HTTP GETs are blocked or require JS-driven flows.
        """
        if sync_playwright is None:
            raise RuntimeError("playwright package is not installed; install with `pip install playwright`")

        with sync_playwright() as p:
            browser_launcher = getattr(p, self.playwright_browser)
            browser = browser_launcher.launch(headless=self.headless)
            context = browser.new_context(user_agent=self.user_agent)
            page = context.new_page()
            try:
                # Try to navigate and read response body (works when server returns body).
                try:
                    resp = page.goto(url, timeout=timeout * 1000, wait_until="commit")
                except Exception:
                    resp = None
                if resp is not None:
                    try:
                        body = resp.body()
                        with open(out_path, "wb") as fh:
                            fh.write(body)
                        return
                    except Exception:
                        # If response body isn't available (e.g. navigation triggers a download),
                        # fall through to handling download events.
                        pass

                # Handle navigation-triggered downloads: try expect_download first
                try:
                    with page.expect_download(timeout=timeout * 1000) as download_info:
                        # If navigation already happened above without body, try to navigate again
                        page.goto(url, timeout=timeout * 1000, wait_until="commit")
                    download = download_info.value
                    # Save to desired path
                    download.save_as(out_path)
                    return
                except Exception:
                    # If navigating raises due to download starting, fallback to Playwright's request API
                    try:
                        req_ctx = p.request.new_context(user_agent=self.user_agent)
                        try:
                            r = req_ctx.get(url, timeout=timeout * 1000)
                            status = getattr(r, "status", None)
                            if status is not None and status >= 400:
                                raise RuntimeError(f"Request failed with status {status}")
                            body = r.body()
                            with open(out_path, "wb") as fh:
                                fh.write(body)
                            return
                        finally:
                            try:
                                req_ctx.dispose()
                            except Exception:
                                pass
                    except Exception:
                        # Last resort: try to wait for any download event after navigation
                        try:
                            dl = page.wait_for_event("download", timeout=3000)
                            dl.save_as(out_path)
                            return
                        except Exception:
                            raise
            finally:
                try:
                    browser.close()
                except Exception:
                    pass

    def fetch_announcements_playwright(self, symbol: str, date_from: str, date_to: str, max_retries: int = 3) -> List[Dict]:
        """Fetch announcements using Playwright's `request` context (bypasses full page navigation).

        This is more robust on sites that block full browser navigation but allow API calls.
        Falls back to `fetch_announcements` if Playwright is not available.
        """
        if sync_playwright is None:
            return self.fetch_announcements(symbol, date_from, date_to, max_retries=max_retries)

        api_url = "https://www.nseindia.com/api/corporate-announcements"
        params = {
            "symbol": symbol,
            "from": date_from,
            "to": date_to,
            "category": "results",
        }

        with sync_playwright() as p:
            # Open a browser context and visit NSE homepage to obtain necessary cookies
            browser_launcher = getattr(p, self.playwright_browser)
            browser = browser_launcher.launch(headless=self.headless)
            context = browser.new_context(user_agent=self.user_agent)
            page = context.new_page()
            try:
                try:
                    page.goto("https://www.nseindia.com/", timeout=15000)
                except Exception:
                    # ignore navigation errors; cookies may still be set
                    pass

                # Build Cookie header from browser context cookies
                cookies = context.cookies()
                cookie_header = "; ".join([f"{c.get('name')}={c.get('value')}" for c in cookies if c.get('name') and c.get('value')])

                # Create a request context including cookie header and referer
                extra_headers = {
                    'Accept': 'application/json, text/javascript, */*; q=0.01',
                    'Referer': 'https://www.nseindia.com/',
                }
                if cookie_header:
                    extra_headers['Cookie'] = cookie_header

                req_ctx = p.request.new_context(user_agent=self.user_agent, extra_http_headers=extra_headers)
                try:
                    resp = req_ctx.get(api_url, params=params, timeout=15000)
                    status = getattr(resp, "status", None)
                    if status is not None and status >= 400:
                        return self.fetch_announcements(symbol, date_from, date_to, max_retries=max_retries)

                    try:
                        data = resp.json()
                    except Exception:
                        return []
                finally:
                    try:
                        req_ctx.dispose()
                    except Exception:
                        pass
            finally:
                try:
                    browser.close()
                except Exception:
                    pass

        # Reuse normalization logic from fetch_announcements
        items = []
        if isinstance(data, dict):
            for key in ("data", "announcements", "results", "rows"):
                if key in data and isinstance(data[key], list):
                    items = data[key]
                    break
            if not items:
                if any(k in data for k in ("symbol", "doc_url", "documentLink", "announcementDate")):
                    items = [data]
        elif isinstance(data, list):
            items = data

        out: List[Dict] = []
        for it in items:
            try:
                doc_url = it.get("documentLink") or it.get("doc_url") or it.get("document_url") or it.get("url")
            except Exception:
                doc_url = None

            report_date = None
            for dkey in ("announcementDate", "date", "report_date", "publishedDate"):
                if isinstance(it.get(dkey), str):
                    report_date = it.get(dkey)
                    break

            out.append({
                "symbol": it.get("symbol") or symbol,
                "report_date": report_date,
                "document_url": doc_url,
                "title": it.get("title") or it.get("announcementTitle"),
                "raw": it,
            })

        return out


__all__ = ["FinancialDownloader", "PlaywrightFinancialDownloader"]
