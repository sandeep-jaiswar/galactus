from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
from typing import List, Dict, Any
import os

from data import provider

app = FastAPI(title="Galactus Research Data Provider")


class OHLCBar(BaseModel):
    timestamp: str
    open: float
    high: float
    low: float
    close: float
    volume: int


@app.get("/v1/historical/{symbol}", response_model=List[OHLCBar])
async def historical(symbol: str, days: int = 30, allow_synthetic: bool = False):
    try:
        # `provider` is the module; use the global data provider instance
        dp = provider.get_data_provider()
        df = dp.get_historical_data(
            symbol.upper(), days=days, allow_synthetic=allow_synthetic
        )
    except RuntimeError as e:
        raise HTTPException(status_code=403, detail=str(e))
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

    if df is None or df.empty:
        raise HTTPException(status_code=404, detail="No historical data available")

    out: List[Dict[str, Any]] = []
    for idx, row in df.iterrows():
        out.append(
            {
                "timestamp": str(idx),
                "open": float(row.get("open", row.get("Open", 0.0))),
                "high": float(row.get("high", row.get("High", 0.0))),
                "low": float(row.get("low", row.get("Low", 0.0))),
                "close": float(row.get("close", row.get("Close", 0.0))),
                "volume": int(row.get("volume", row.get("Volume", 0))),
            }
        )

    return out
