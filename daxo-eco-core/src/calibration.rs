const GAMMA: f32 = 0.2;

pub fn calibrate_pm(raw_pm: f32, rh: f32) -> f32 {
    if rh <= 50.0 {
        raw_pm
    } else {
        let correction = 1.0 + GAMMA * (rh - 50.0) / 100.0;
        raw_pm / correction
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_calibration() {
        let raw = 40.0;
        let corrected = calibrate_pm(raw, 60.0);
        assert!(corrected < raw);
        assert!((corrected - 38.46).abs() < 0.01);
    }
}