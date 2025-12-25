import requests
from datetime import datetime
import os

def download_bhavcopy(session_date: str) -> str:
    """
    session_date: YYYY-MM-DD
    returns local CSV path
    """

    date_obj = datetime.strptime(session_date, "%Y-%m-%d")
    session_date_str = date_obj.strftime("%d%m%Y")
    url = f"https://nsearchives.nseindia.com/products/content/sec_bhavdata_full_{session_date_str}.csv"
    output_dir = f"/tmp/bhavcopy/{session_date}"
    os.makedirs(output_dir, exist_ok=True)

    headers = {
        "User-Agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36",
        "Accept": "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8",
        "Accept-Language": "en-US,en;q=0.5",
        "Accept-Encoding": "gzip, deflate",
        "Connection": "keep-alive",
        "Upgrade-Insecure-Requests": "1",
    }

    file_path = f"{output_dir}/sec_bhavdata_{session_date}.csv"
    response = requests.get(url, headers=headers, timeout=30)
    if response.status_code == 200:
        with open(file_path, 'wb') as f:
            f.write(response.content)
        return file_path
    else:
        raise Exception(f"Failed to download bhavcopy for {session_date}, status code: {response.status_code}")
