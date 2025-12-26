"""Helpers to fetch and extract financial results from NSE corporate filings.

This module uses Playwright (if available) to perform authenticated API calls
by first visiting the NSE homepage to obtain cookies, then calling the
`corporates-financial-results` endpoint. It provides `get_financial_results_master`
which returns a master DataFrame and `financial_results_for_equity` which
parses XBRL content from returned rows into a flattened DataFrame.

These functions are defensive: if Playwright is not available they fall back
to `requests` where possible.
"""
from typing import Tuple, List
import json
import pandas as pd
import requests
import xml.etree.ElementTree as ET
from datetime import datetime, timedelta
import concurrent.futures
import threading
import time
import logging
from utils.financial_parser import parse_xbrl

try:
    from playwright.sync_api import sync_playwright
except Exception:
    sync_playwright = None


class NSEdataNotFound(Exception):
    pass


def validate_date_param(from_date: str, to_date: str, period: str):
    # minimal validation: either from/to present or period present
    if (not from_date or not to_date) and not period:
        raise ValueError("Provide either from_date+to_date or period")


def derive_from_and_to_date(from_date: str = None, to_date: str = None, period: str = None) -> Tuple[str, str]:
    # returns dates in dd-mm-YYYY
    if period and (not from_date and not to_date):
        # support simple period tokens
        today = datetime.utcnow().date()
        if period == '1D':
            dt_from = today - timedelta(days=1)
        elif period == '1W':
            dt_from = today - timedelta(days=7)
        elif period == '1M':
            dt_from = today - timedelta(days=30)
        elif period == '6M':
            dt_from = today - timedelta(days=182)
        elif period == '1Y':
            dt_from = today - timedelta(days=365)
        else:
            raise ValueError('Unsupported period token')
        dt_to = today
        return dt_from.strftime('%d-%m-%Y'), dt_to.strftime('%d-%m-%Y')

    # assume provided in dd-mm-YYYY — basic validation
    def _norm(d):
        if d is None:
            return None
        try:
            datetime.strptime(d, '%d-%m-%Y')
            return d
        except Exception:
            raise ValueError('Dates must be in dd-mm-YYYY')

    return _norm(from_date), _norm(to_date)


def _fetch_json_via_playwright(api_url: str, params: dict, user_agent: str = None, timeout: int = 15000) -> Tuple[int, dict]:
    headers = {
        'Accept': 'application/json, text/javascript, */*; q=0.01',
        'Referer': 'https://www.nseindia.com/',
    }

    # Prefer Playwright to bootstrap cookies and perform request
    if sync_playwright is None:
        # fallback to requests
        r = requests.get(api_url, params=params, headers={'User-Agent': user_agent or requests.utils.default_headers().get('User-Agent'), **headers}, timeout=timeout/1000)
        try:
            return r.status_code, r.json()
        except Exception:
            return r.status_code, {}

    with sync_playwright() as p:
        browser = p.chromium.launch(headless=True)
        context = browser.new_context(user_agent=user_agent or 'Mozilla/5.0')
        page = context.new_page()
        try:
            try:
                page.goto('https://www.nseindia.com/', timeout=timeout)
            except Exception:
                pass
            # construct cookie header
            cookies = context.cookies()
            cookie_header = '; '.join([f"{c.get('name')}={c.get('value')}" for c in cookies if c.get('name') and c.get('value')])
            extra_headers = headers.copy()
            if cookie_header:
                extra_headers['Cookie'] = cookie_header
            req_ctx = p.request.new_context(user_agent=user_agent or 'Mozilla/5.0', extra_http_headers=extra_headers)
            try:
                resp = req_ctx.get(api_url, params=params, timeout=timeout)
                status = getattr(resp, 'status', None)
                try:
                    js = resp.json()
                except Exception:
                    js = {}
                return status, js
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


def get_financial_results_master(from_date: str = None,
                                 to_date: str = None,
                                 period: str = None,
                                 fo_sec: bool = False,
                                 fin_period: str = 'Quarterly') -> Tuple[pd.DataFrame, dict, dict, List[str]]:
    validate_date_param(from_date, to_date, period)
    from_date, to_date = derive_from_and_to_date(from_date=from_date, to_date=to_date, period=period)
    api_url = "https://www.nseindia.com/api/corporates-financial-results"
    params = {'index': 'equities'}
    if fo_sec:
        params.update({'from_date': from_date, 'to_date': to_date, 'fo_sec': 'true', 'period': fin_period})
    else:
        params.update({'from_date': from_date, 'to_date': to_date, 'period': fin_period})

    status, js = _fetch_json_via_playwright(api_url, params)
    if status != 200:
        raise NSEdataNotFound('Resource not available for financial data with these parameters')
    data_list = js if isinstance(js, list) else js.get('data', []) if isinstance(js, dict) else []
    master_data_df = pd.DataFrame(data_list)
    if master_data_df.empty:
        return master_data_df, {}, {}, []
    master_data_df.columns = [name.replace(' ', '') for name in master_data_df.columns]
    headers = {
        'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3',
        'Accept': '*/*',
        'Accept-Language': 'en-US,en;q=0.9',
        'Referer': 'https://www.nseindia.com/'
    }
    ns = {
        "xbrli": "http://www.xbrl.org/2003/instance",
        "in-bse-fin": "http://www.bseindia.com/xbrl/fin/2020-03-31/in-bse-fin"
    }
    keys_to_extract = [
        "ScripCode", "Symbol", "MSEISymbol", "NameOfTheCompany", "ClassOfSecurity",
        "DateOfStartOfFinancialYear", "DateOfEndOfFinancialYear",
        "DateOfBoardMeetingWhenFinancialResultsWereApproved",
        "DateOnWhichPriorIntimationOfTheMeetingForConsideringFinancialResultsWasInformedToTheExchange",
        "DescriptionOfPresentationCurrency", "LevelOfRoundingUsedInFinancialStatements",
        "ReportingQuarter", "StartTimeOfBoardMeeting", "EndTimeOfBoardMeeting",
        "DateOfStartOfBoardMeeting", "DateOfEndOfBoardMeeting",
        "DeclarationOfUnmodifiedOpinionOrStatementOnImpactOfAuditQualification",
        "IsCompanyReportingMultisegmentOrSingleSegment", "DescriptionOfSingleSegment",
        "DateOfStartOfReportingPeriod", "DateOfEndOfReportingPeriod",
        "WhetherResultsAreAuditedOrUnaudited", "NatureOfReportStandaloneConsolidated",
        "RevenueFromOperations", "OtherIncome", "Income", "CostOfMaterialsConsumed",
        "PurchasesOfStockInTrade", "ChangesInInventoriesOfFinishedGoodsWorkInProgressAndStockInTrade",
        "EmployeeBenefitExpense", "FinanceCosts", "DepreciationDepletionAndAmortisationExpense",
        "OtherExpenses", "Expenses", "ProfitBeforeExceptionalItemsAndTax", "ExceptionalItemsBeforeTax",
        "ProfitBeforeTax", "CurrentTax", "DeferredTax", "TaxExpense",
        "NetMovementInRegulatoryDeferralAccountBalancesRelatedToProfitOrLossAndTheRelatedDeferredTaxMovement",
        "ProfitLossForPeriodFromContinuingOperations", "ProfitLossFromDiscontinuedOperationsBeforeTax",
        "TaxExpenseOfDiscontinuedOperations", "ProfitLossFromDiscontinuedOperationsAfterTax",
        "ShareOfProfitLossOfAssociatesAndJointVenturesAccountedForUsingEquityMethod",
        "ProfitLossForPeriod", "OtherComprehensiveIncomeNetOfTaxes",
        "ComprehensiveIncomeForThePeriod", "ProfitOrLossAttributableToOwnersOfParent",
        "ProfitOrLossAttributableToNonControllingInterests",
        "ComprehensiveIncomeForThePeriodAttributableToOwnersOfParent",
        "ComprehensiveIncomeForThePeriodAttributableToOwnersOfParentNonControllingInterests",
        "PaidUpValueOfEquityShareCapital", "FaceValueOfEquityShareCapital",
        "BasicEarningsLossPerShareFromContinuingOperations",
        "DilutedEarningsLossPerShareFromContinuingOperations",
        "BasicEarningsLossPerShareFromDiscontinuedOperations",
        "DilutedEarningsLossPerShareFromDiscontinuedOperations",
        "BasicEarningsLossPerShareFromContinuingAndDiscontinuedOperations",
        "DilutedEarningsLossPerShareFromContinuingAndDiscontinuedOperations",
        "DescriptionOfOtherExpenses", "OtherExpenses",
        "DescriptionOfItemThatWillNotBeReclassifiedToProfitAndLoss",
        "AmountOfItemThatWillNotBeReclassifiedToProfitOrLoss",
        "IncomeTaxRelatingToItemsThatWillNotBeReclassifiedToProfitOrLoss",
        "DescriptionOfItemThatWillBeReclassifiedToProfitAndLoss",
        "AmountOfItemThatWillBeReclassifiedToProfitOrLoss",
        "IncomeTaxRelatingToItemsThatWillBeReclassifiedToProfitOrLoss"
    ]
    return master_data_df, headers, ns, keys_to_extract


def financial_results_for_equity(from_date: str = None,
                                 to_date: str = None,
                                 period: str = None,
                                 fo_sec: bool = False,
                                 fin_period: str = 'Quarterly') -> pd.DataFrame:
    master_data_df, headers, ns, keys_to_extract = get_financial_results_master(from_date, to_date, period,
                                                                                fo_sec, fin_period)
    if master_data_df.empty:
        return pd.DataFrame()

    # Collect valid rows with xbrl links
    rows = []
    for row in master_data_df.to_dict(orient='records'):
        xbrl_url = None
        for candidate in ('xbrl', 'xbrlLink', 'Xbrl', 'xbrl_link', 'documentLink'):
            if candidate in row and row[candidate]:
                xbrl_url = row[candidate]
                break
        if not xbrl_url:
            continue
        if isinstance(xbrl_url, str) and (xbrl_url.endswith('/-') or xbrl_url.strip().endswith('-') or xbrl_url.strip() == '-'):
            continue
        rows.append((row, xbrl_url))

    # Bootstrap a single requests.Session using Playwright cookies (if possible)
    session = requests.Session()
    session.headers.update(headers)
    if sync_playwright is not None:
        try:
            with sync_playwright() as p:
                browser = p.chromium.launch(headless=True)
                ctx = browser.new_context(user_agent=headers.get('User-Agent'))
                page = ctx.new_page()
                try:
                    try:
                        page.goto('https://www.nseindia.com/', timeout=15000)
                    except Exception:
                        pass
                    for c in ctx.cookies():
                        session.cookies.set(c.get('name'), c.get('value'), domain=c.get('domain'), path=c.get('path'))
                finally:
                    try:
                        browser.close()
                    except Exception:
                        pass
        except Exception:
            # ignore bootstrap failures and continue with plain session
            pass

    # Worker: fetch and parse one XBRL document using the shared session
    def _fetch_parse(item):
        row, url = item
        try:
            r = session.get(url, timeout=30)
            r.raise_for_status()
            content = r.content
            # Use parser to convert XBRL bytes into canonical record
            rec = parse_xbrl(content)
            if not isinstance(rec, dict):
                return None
            rec['source_url'] = url
            # Ensure symbol exists
            if not rec.get('symbol'):
                rec['symbol'] = row.get('symbol')
            return rec
        except Exception as e:
            logging.debug('Failed to fetch/parse XBRL %s: %s', url, e)
            return None

    results = []
    if not rows:
        return pd.DataFrame()

    max_workers = min(16, max(4, (len(rows) // 50) + 1))
    with concurrent.futures.ThreadPoolExecutor(max_workers=max_workers) as exe:
        for res in exe.map(_fetch_parse, rows):
            if res:
                results.append(res)

    if not results:
        return pd.DataFrame()
    return pd.DataFrame(results)


__all__ = ['get_financial_results_master', 'financial_results_for_equity']
