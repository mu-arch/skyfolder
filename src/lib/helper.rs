pub fn format_file_size_pretty(size: Option<u64>) -> Option<String> {
    let units = vec!["b", "kb", "mb", "gb", "tb"];
    let kib = 1024.0;

    let bytes = match size {
        None => return None,
        Some(v) => v as f64
    };

    if bytes == 0.0 || bytes.is_nan() {
        return None;
    }

    let base = f64::log(bytes, kib).floor() as usize;
    if base >= units.len() {
        return Some("Too Large".to_string());
    }

    let adjusted_bytes = bytes / f64::powi(kib, base as i32);
    let unit = &units[base];

    if f64::round(adjusted_bytes.fract() * 10.0) == 0.0 {
        return Some(format!("{} <span>{}</span>", adjusted_bytes.floor(), unit));
    } else {
        let formatted_adjusted_bytes = format!("{:.1}", adjusted_bytes);
        return Some(format!("{} <span>{}</span>", formatted_adjusted_bytes.trim_end_matches(".0"), unit));
    }
}