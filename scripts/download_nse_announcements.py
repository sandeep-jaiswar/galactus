#!/usr/bin/env python3
"""CLI to fetch NSE corporate announcements for a symbol/date range and download documents."""
import argparse
import os
from pathlib import Path
from utils.financial_download import PlaywrightFinancialDownloader


def main():
    p = argparse.ArgumentParser(description="Download NSE announcements documents for a symbol")
    p.add_argument("symbol")
    p.add_argument("from_date", help="from date in dd-mm-YYYY")
    p.add_argument("to_date", help="to date in dd-mm-YYYY")
    p.add_argument("--out-dir", default="/tmp/nse_announcements")
    p.add_argument("--headless", action="store_true", default=True)
    p.add_argument("--use-playwright-bootstrap", action="store_true", default=False,
                   help="Use Playwright to bootstrap browser session (may be required for some sites)")
    args = p.parse_args()

    out_dir = Path(args.out_dir)
    out_dir.mkdir(parents=True, exist_ok=True)

    dl = PlaywrightFinancialDownloader(headless=args.headless)

    # Use Playwright request API to fetch announcements (more robust)
    ann = dl.fetch_announcements_playwright(args.symbol, args.from_date, args.to_date)
    if not ann:
        print("No announcements returned")
        return

    print(f"Found {len(ann)} announcements; attempting downloads")
    for i, a in enumerate(ann, 1):
        url = a.get("document_url")
        if not url:
            print(f"{i}: no document URL; skipping: {a.get('title')}")
            continue
        # sanitize filename
        fn = f"{a.get('symbol')}_{a.get('report_date') or 'unknown'}_{i}.pdf"
        fn = "".join(c if c.isalnum() or c in ("-", "_") else "_" for c in fn)
        out_path = out_dir / fn
        print(f"{i}: downloading {url} -> {out_path}")
        try:
            # Try requests-based download first (uses underlying session cookies if bootstrapped)
            dl.download_document(url, str(out_path))
            print("  -> ok")
        except Exception as e:
            print("  -> requests download failed, falling back to Playwright: ", e)
            try:
                dl.download_document_playwright(url, str(out_path))
                print("  -> ok (playwright)")
            except Exception as e2:
                print("  -> failed (playwright)", e2)


if __name__ == "__main__":
    main()
