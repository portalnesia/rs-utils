/*
 * Copyright (c) Portalnesia - All Rights Reserved
 * Unauthorized copying of this file, via any medium is strictly prohibited
 * Proprietary and confidential
 * Written by Putu Aditya <aditya@portalnesia.com>
 */

use serde::{Deserialize, Serialize};

/// bytes_format_function to format bytes to human-readable string
///
/// Example: 50486525485 => "5.05 GB"
pub fn bytes_format_function(bytes: f64, precision: usize) -> String {
    // Daftar unit binary
    const UNITS: [&str; 7] = ["B", "KB", "MB", "GB", "TB", "PB", "EB"];

    let mut size = bytes;
    let mut idx = 0;
    // Bagi dengan 1024 sampai <1024 atau unit terakhir
    while size >= 1024.0 && idx < UNITS.len() - 1 {
        size /= 1024.0;
        idx += 1;
    }

    // Format dengan precision dinamis
    format!("{:.*} {}", precision, size, UNITS[idx])
}

/// bytes_format to format bytes to human-readable string
///
/// Example: 50486525485 => "5.05 GB"
#[macro_export]
macro_rules! bytes_format {
    ($bytes:tt) => {
        $crate::helper::bytes_format_function($bytes, 2)
    };
    ($bytes:tt,$precision:tt) => {
        $crate::helper::bytes_format_function($bytes, $precision)
    };
}

pub fn format_number_function(
    value: f64,
    precision: usize,
    thousand_sep: char,
    decimal_sep: char,
) -> String {
    // 1) Buat string dengan precision yang diinginkan
    let s = format!("{:.*}", precision, value);
    // 2) Split integer dan fractional part
    let parts = s.split('.').collect::<Vec<_>>();
    let int_part = parts[0];
    let frac_part = if precision > 0 && parts.len() > 1 {
        parts[1]
    } else {
        ""
    };

    // 3) Sisipkan thousand_sep setiap 3 digit dari kanan
    let mut rev = int_part.chars().rev().peekable();
    let mut formatted_int = String::new();
    for (i, c) in rev.by_ref().enumerate() {
        if i > 0 && i % 3 == 0 {
            formatted_int.push(thousand_sep);
        }
        formatted_int.push(c);
    }
    let formatted_int = formatted_int.chars().rev().collect::<String>();

    // 4) Gabungkan lagi dengan fractional part (jika ada)
    if precision > 0 {
        format!("{}{}{}", formatted_int, decimal_sep, frac_part)
    } else {
        formatted_int
    }
}

/// Format float64 with a thousand separator.
///
/// Default:
/// - precision: 0
/// - thousand separator: .
/// - decimal separator: ,
///
/// Arguments:
/// - value: f64
/// - precision: usize
/// - thousand_sep: char
/// - decimal_sep: char
///
/// Example
///
/// ```
/// utils::format_number!(5000.0);
/// utils::format_number!(1234567.89, 2);
/// utils::format_number!(5000.0, 0, ',', '.');
/// ```
#[macro_export]
macro_rules! format_number {
    ($value:tt) => {
        $crate::helper::format_number_function($value, 0, '.', ',')
    };
    ($value:tt,$precision:tt) => {
        $crate::helper::format_number_function($value, $precision, '.', ',')
    };
    ($value:tt,$precision:tt,$thousand_sep:expr,$decimal_sep:expr) => {
        $crate::helper::format_number_function($value, $precision, $thousand_sep, $decimal_sep)
    };
}

#[derive(Serialize, Deserialize)]
pub struct FormatShortResponse {
    number: f64,
    format: String,
}

/// Ubah angka jadi format singkat dengan suffix:
/// - K untuk ribu (10³)
/// - M untuk juta (10⁶)
/// - B untuk miliar (10⁹)
///
/// Jika < 1 000, dikembalikan apa adanya (tanpa suffix).
///
/// Precision mengatur jumlah angka di belakang koma.
pub fn format_short_number(value: f64, precision: usize) -> FormatShortResponse {
    // ambil nilai absolut untuk threshold, tapi pertahankan tanda asli
    let sign = if value.is_sign_negative() { "-" } else { "" };
    let abs = value.abs();

    let (scaled, suffix) = if abs >= 1_000_000_000.0 {
        (abs / 1_000_000_000.0, "B")
    } else if abs >= 1_000_000.0 {
        (abs / 1_000_000.0, "M")
    } else if abs >= 1_000.0 {
        (abs / 1_000.0, "K")
    } else {
        (abs, "")
    };

    let mut resp = FormatShortResponse {
        number: value,
        format: String::new(),
    };
    // format dengan precision, tambahkan suffix jika ada
    if suffix.is_empty() {
        resp.format = format!("{}{:.*}", sign, precision, scaled);
    } else {
        resp.format = format!("{}{:.*} {}", sign, precision, scaled, suffix);
    }
    resp
}

#[cfg(test)]
mod tests {
    // use super::*;

    use crate::helper::format_short_number;

    #[test]
    fn test_bytes_format_various() {
        let cases = vec![
            // bytes, precision, expected
            (0.0, 2, "0.00 B"),
            (500.0, 0, "500 B"),
            (1024.0, 1, "1.0 KB"),
            (1536.0, 2, "1.50 KB"),                 // 1536 / 1024 = 1.5
            (1_048_576.0, 2, "1.00 MB"),            // 1024^2
            (1_073_741_824.0, 2, "1.00 GB"),        // 1024^3
            (5_368_709_120.0, 1, "5.0 GB"),         // 5 * 1024^3
            (1_099_511_627_776.0, 3, "1.000 TB"),   // 1024^4
            (50_000_000_000.0, 2, "46.57 GB"),      // ~46.566 GB
            (5_629_499_534_213_120.0, 1, "5.0 PB"), // 5 * 1024^5
            (50486525485f64, 2, "47.02 GB"),
            (18037807f64, 2, "17.20 MB"),
            (0f64, 0, "0 B"),
        ];

        for (bytes, precision, expected) in cases {
            let got = bytes_format!(bytes, precision);
            assert_eq!(
                got, expected,
                "bytes_format({}, {}) should be {:?}, got {:?}",
                bytes, precision, expected, got
            );
        }
    }

    #[test]
    fn test_format_number() {
        let cases = vec![
            // (value, precision, thou, dec, expected)
            (5000.0, 0, '.', ',', "5.000"),
            (1234.0, 0, ',', '.', "1,234"),
            (1234567.89, 2, '.', ',', "1.234.567,89"),
            (1234567.89, 2, ',', '.', "1,234,567.89"),
            (100.5, 1, '.', ',', "100,5"),
            (100.0, 3, ',', '.', "100.000"),
            (0.0, 2, '.', ',', "0,00"),
        ];

        for (value, prec, thou, dec, expected) in cases {
            let got = format_number!(value, prec, thou, dec);
            assert_eq!(
                got, expected,
                "format_number({}, {}, '{}', '{}') -> {:?}, expected {:?}",
                value, prec, thou, dec, got, expected
            );
        }

        let n = format_number!(1000.0);
        assert_eq!(n, "1.000");

        let n = format_number!(1000.0, 2);
        assert_eq!(n, "1.000,00");

        let n = format_number!(1000.0, 2, ',', '.');
        assert_eq!(n, "1,000.00")
    }

    #[test]
    fn test_format_short_number() {
        let cases = vec![
            // (value, precision, expected)
            (5025.0, 2, "5.03 K"),
            (64768456.0, 2, "64.77 M"),
            (1_065_201_025.0, 2, "1.07 B"),
            (999.0, 2, "999.00"), // tanpa suffix
            (1000.0, 1, "1.0 K"),
            (1_000_000.0, 0, "1 M"),
            (1_500_000_000.0, 3, "1.500 B"),
            (-2500.0, 2, "-2.50 K"), // negatif
            (0.0, 2, "0.00"),
        ];

        for (value, precision, expected) in cases {
            let got = format_short_number(value, precision);
            assert_eq!(
                got.format, expected,
                "format_short_number({}, {}) should be {:?}, got {:?}",
                value, precision, expected, got.format
            );
        }
    }
}
