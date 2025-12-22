from nsemine.bhavcopy import BhavCopy
from datetime import datetime
import os

def download_bhavcopy(session_date: str, output_dir: str) -> str:
    """
    session_date: YYYY-MM-DD
    returns local CSV path
    """

    date_obj = datetime.strptime(session_date, "%Y-%m-%d")
    output_dir = f"/tmp/bhavcopy/{session_date}"
    os.makedirs(output_dir, exist_ok=True)

    bc = BhavCopy(date_obj)
    df = bc.get_data()

    file_path = f"{output_dir}/sec_bhavdata_{session_date}.csv"
    df.to_csv(file_path, index=False)

    return file_path
