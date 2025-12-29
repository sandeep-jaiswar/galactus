"""
Galactus Leading Indicators Module

Provides early signals for regime transitions before structural breaks occur.
Reduces regime lag by detecting:
1. Futures-Spot Basis Divergence (>50bps = stress indicator)
2. Put/Call OI Ratio Extremes (>1.3 = hedging pressure)
3. OI Concentration Changes (>15% in single strike = compression)

These signals anticipate regime changes 1-2 days early.
"""

from dataclasses import dataclass
from datetime import datetime
from typing import Optional, Dict, List, Tuple
import logging

logger = logging.getLogger(__name__)


@dataclass
class LeadingIndicatorSignal:
    """Signal from a leading indicator"""

    signal_type: str  # "basis_divergence", "put_call_extreme", "oi_concentration"
    timestamp: datetime
    strength: float  # 0-1, confidence of signal
    implied_regime: str  # What regime this suggests is coming
    confidence: float  # 0-1, how confident is this prediction
    days_to_transition: int  # Estimated days until regime change
    supporting_data: Dict = None


@dataclass
class LeadingIndicatorsResult:
    """Aggregated leading indicators analysis"""

    regime_transition_likely: bool
    suggested_regime: Optional[str]
    combined_strength: float  # 0-1, aggregated signal strength
    individual_signals: List[LeadingIndicatorSignal]
    recommendation: str  # Action to take based on signals
    timestamp: datetime


class BasisDivergenceDetector:
    """
    Detect futures-spot basis divergence indicating regime stress.

    Normal basis: 0-30bps (fair value premium)
    Stressed basis: >50bps (suggests forced selling or hedging pressure)
    Extreme: >100bps (market dislocation)
    """

    NORMAL_BASIS_BPS = 30  # Normal futures premium
    STRESS_THRESHOLD_BPS = 50
    EXTREME_THRESHOLD_BPS = 100

    @staticmethod
    def calculate_basis_divergence(
        futures_price: float,
        spot_price: float,
        time_to_expiry_days: float,
        risk_free_rate: float = 0.065,
    ) -> Tuple[float, float]:
        """
        Calculate actual vs fair basis.

        Fair basis ≈ spot * (1 + r*t) where r=risk-free rate, t=time fraction

        Args:
            futures_price: Current futures price
            spot_price: Current spot price
            time_to_expiry_days: Days until expiry
            risk_free_rate: Annual risk-free rate

        Returns:
            (actual_basis_bps, fair_basis_bps)
        """
        actual_basis = futures_price - spot_price
        actual_basis_bps = (actual_basis / spot_price) * 10000

        # Fair basis calculation
        time_fraction = time_to_expiry_days / 365.0
        fair_basis = spot_price * (risk_free_rate * time_fraction)
        fair_basis_bps = (fair_basis / spot_price) * 10000

        return actual_basis_bps, fair_basis_bps

    @staticmethod
    def detect_divergence(
        futures_price: float,
        spot_price: float,
        time_to_expiry_days: float,
        risk_free_rate: float = 0.065,
    ) -> Optional[LeadingIndicatorSignal]:
        """
        Detect if basis divergence signals regime stress.
        """
        actual_basis_bps, fair_basis_bps = (
            BasisDivergenceDetector.calculate_basis_divergence(
                futures_price, spot_price, time_to_expiry_days, risk_free_rate
            )
        )

        divergence_bps = abs(actual_basis_bps - fair_basis_bps)

        if divergence_bps > BasisDivergenceDetector.STRESS_THRESHOLD_BPS:
            # Divergence detected
            if divergence_bps > BasisDivergenceDetector.EXTREME_THRESHOLD_BPS:
                strength = 1.0
                confidence = 0.95
            else:
                strength = (
                    divergence_bps - BasisDivergenceDetector.STRESS_THRESHOLD_BPS
                ) / 50
                confidence = 0.85

            return LeadingIndicatorSignal(
                signal_type="basis_divergence",
                timestamp=datetime.utcnow(),
                strength=strength,
                implied_regime=(
                    "Expiry Compression"
                    if divergence_bps > 0
                    else "Elevated Volatility"
                ),
                confidence=confidence,
                days_to_transition=1,
                supporting_data={
                    "actual_basis_bps": actual_basis_bps,
                    "fair_basis_bps": fair_basis_bps,
                    "divergence_bps": divergence_bps,
                },
            )

        return None


class PutCallRatioDetector:
    """
    Detect put/call OI ratio extremes indicating hedging pressure.

    Normal ratio: 0.8-1.2 (balanced)
    Call heavy: <0.8 (bullish, low hedging)
    Put heavy: >1.3 (hedging pressure, regime uncertainty)
    Extreme: >1.5 (panic hedging)
    """

    NORMAL_RATIO_MIN = 0.8
    NORMAL_RATIO_MAX = 1.2
    PUT_EXTREME_THRESHOLD = 1.3
    PANIC_THRESHOLD = 1.5

    @staticmethod
    def calculate_ratio(calls_oi: Dict[float, int], puts_oi: Dict[float, int]) -> float:
        """
        Calculate put/call OI ratio.

        Args:
            calls_oi: Dictionary of {strike: call_oi}
            puts_oi: Dictionary of {strike: put_oi}

        Returns:
            Total puts OI / Total calls OI
        """
        total_calls = sum(calls_oi.values()) if calls_oi else 1
        total_puts = sum(puts_oi.values()) if puts_oi else 1

        return total_puts / total_calls if total_calls > 0 else 1.0

    @staticmethod
    def detect_extreme_ratio(
        calls_oi: Dict[float, int],
        puts_oi: Dict[float, int],
    ) -> Optional[LeadingIndicatorSignal]:
        """
        Detect if put/call ratio signals hedging pressure.
        """
        ratio = PutCallRatioDetector.calculate_ratio(calls_oi, puts_oi)

        if ratio > PutCallRatioDetector.PUT_EXTREME_THRESHOLD:
            # Put heavy - hedging pressure
            if ratio > PutCallRatioDetector.PANIC_THRESHOLD:
                strength = 1.0
                confidence = 0.92
                regime = "Elevated Volatility"
            else:
                strength = (ratio - PutCallRatioDetector.PUT_EXTREME_THRESHOLD) / 0.2
                confidence = 0.85
                regime = "Elevated Volatility"

            return LeadingIndicatorSignal(
                signal_type="put_call_extreme",
                timestamp=datetime.utcnow(),
                strength=strength,
                implied_regime=regime,
                confidence=confidence,
                days_to_transition=1,
                supporting_data={
                    "put_call_ratio": ratio,
                    "total_calls": sum(calls_oi.values()) if calls_oi else 0,
                    "total_puts": sum(puts_oi.values()) if puts_oi else 0,
                },
            )

        return None


class OIConcentrationDetector:
    """
    Detect OI concentration changes indicating expiry compression.

    Normal concentration: <10% in largest strike
    Building pressure: 10-15% (expiry approaching)
    Extreme compression: >15% (expiry imminent)
    """

    BUILDING_THRESHOLD = 0.10
    COMPRESSION_THRESHOLD = 0.15

    @staticmethod
    def calculate_concentration(
        option_ois: Dict[float, int],
    ) -> Tuple[float, float]:
        """
        Calculate OI concentration.

        Args:
            option_ois: Dictionary of {strike: oi}

        Returns:
            (largest_strike_pct, top_3_strikes_pct)
        """
        if not option_ois or sum(option_ois.values()) == 0:
            return 0.0, 0.0

        total_oi = sum(option_ois.values())
        sorted_ois = sorted(option_ois.values(), reverse=True)

        largest_pct = sorted_ois[0] / total_oi if total_oi > 0 else 0
        top_3_pct = sum(sorted_ois[:3]) / total_oi if total_oi > 0 else 0

        return largest_pct, top_3_pct

    @staticmethod
    def detect_compression(
        call_ois: Dict[float, int],
        put_ois: Dict[float, int],
    ) -> Optional[LeadingIndicatorSignal]:
        """
        Detect if OI concentration signals expiry compression coming.
        """
        calls_largest, calls_top3 = OIConcentrationDetector.calculate_concentration(
            call_ois
        )
        puts_largest, puts_top3 = OIConcentrationDetector.calculate_concentration(
            put_ois
        )

        # Average concentration
        avg_largest = (calls_largest + puts_largest) / 2

        if avg_largest > OIConcentrationDetector.COMPRESSION_THRESHOLD:
            # Compression ahead
            strength = (
                avg_largest - OIConcentrationDetector.COMPRESSION_THRESHOLD
            ) / 0.10
            confidence = 0.90

            return LeadingIndicatorSignal(
                signal_type="oi_concentration",
                timestamp=datetime.utcnow(),
                strength=min(1.0, strength),
                implied_regime="Expiry Compression",
                confidence=confidence,
                days_to_transition=1,
                supporting_data={
                    "calls_largest_pct": calls_largest,
                    "puts_largest_pct": puts_largest,
                    "calls_top_3_pct": calls_top3,
                    "puts_top_3_pct": puts_top3,
                },
            )

        return None


class LeadingIndicatorsAnalyzer:
    """
    Aggregate leading indicators to predict regime transitions.
    """

    def __init__(self):
        self.basis_detector = BasisDivergenceDetector()
        self.putcall_detector = PutCallRatioDetector()
        self.concentration_detector = OIConcentrationDetector()
        self.signal_history = []

    def analyze_all_indicators(
        self,
        futures_price: float,
        spot_price: float,
        time_to_expiry_days: float,
        calls_oi: Dict[float, int],
        puts_oi: Dict[float, int],
        risk_free_rate: float = 0.065,
    ) -> LeadingIndicatorsResult:
        """
        Analyze all leading indicators and return aggregated result.
        """
        signals = []

        # Check basis divergence
        basis_signal = self.basis_detector.detect_divergence(
            futures_price, spot_price, time_to_expiry_days, risk_free_rate
        )
        if basis_signal:
            signals.append(basis_signal)

        # Check put/call ratio
        putcall_signal = self.putcall_detector.detect_extreme_ratio(calls_oi, puts_oi)
        if putcall_signal:
            signals.append(putcall_signal)

        # Check OI concentration
        concentration_signal = self.concentration_detector.detect_compression(
            calls_oi, puts_oi
        )
        if concentration_signal:
            signals.append(concentration_signal)

        # Aggregate signals
        if not signals:
            return LeadingIndicatorsResult(
                regime_transition_likely=False,
                suggested_regime=None,
                combined_strength=0.0,
                individual_signals=[],
                recommendation="No early warning signs detected. Continue monitoring.",
                timestamp=datetime.utcnow(),
            )

        # Combine signals
        avg_strength = sum(s.strength for s in signals) / len(signals)
        avg_confidence = sum(s.confidence for s in signals) / len(signals)

        # Majority regime from signals
        regime_counts = {}
        for signal in signals:
            regime = signal.implied_regime
            regime_counts[regime] = regime_counts.get(regime, 0) + 1

        suggested_regime = max(regime_counts.items(), key=lambda x: x[1])[0]

        # Build recommendation
        if avg_strength > 0.8:
            recommendation = f"⚠️ STRONG signals for {suggested_regime} within 1 day. Reduce confidence."
        elif avg_strength > 0.6:
            recommendation = f"⚠️ MODERATE signals for {suggested_regime} within 1-2 days. Watch closely."
        else:
            recommendation = (
                f"⚠️ WEAK signals for {suggested_regime}. Continue monitoring."
            )

        result = LeadingIndicatorsResult(
            regime_transition_likely=len(signals) > 0,
            suggested_regime=suggested_regime,
            combined_strength=min(1.0, avg_strength),
            individual_signals=signals,
            recommendation=recommendation,
            timestamp=datetime.utcnow(),
        )

        self.signal_history.append(result)
        return result


def create_analyzer() -> LeadingIndicatorsAnalyzer:
    """Factory function to create analyzer instance."""
    return LeadingIndicatorsAnalyzer()
