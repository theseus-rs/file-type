use std::fmt::Formatter;

/// A trait for generating source code.
pub trait Source {
    /// Generate the source code.
    fn to_source(&self) -> String;
}

/// Implement the `Source` trait for `usize`.  Introduces '_' separators for large numbers.
impl Source for usize {
    fn to_source(&self) -> String {
        // Limit the maximum value to u32::MAX to support 32-bit systems.
        let value = if *self > u32::MAX as usize {
            u32::MAX as usize
        } else {
            *self
        };
        value
            .to_string()
            .chars()
            .rev()
            .enumerate()
            .fold(String::new(), |acc, (index, c)| {
                if index % 3 == 0 && index > 0 {
                    format!("{c}_{acc}")
                } else {
                    format!("{c}{acc}")
                }
            })
    }
}

/// Implement the `Source` trait for `[u8]`.  Prints the bytes as a list of hexadecimal values.
impl Source for [u8] {
    fn to_source(&self) -> String {
        let bytes = self
            .iter()
            .map(|byte| format!("0x{byte:02X}"))
            .collect::<Vec<String>>()
            .join(", ");
        format!("[{bytes}]")
    }
}
