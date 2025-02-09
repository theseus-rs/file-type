use crate::format::FileFormat;
use std::slice::Iter;

#[cfg(feature = "custom")]
#[doc(hidden)]
pub mod custom;
#[doc(hidden)]
pub mod default;
#[cfg(feature = "httpd")]
#[doc(hidden)]
pub mod httpd;
#[cfg(feature = "iana")]
#[doc(hidden)]
pub mod iana;
#[cfg(feature = "linguist")]
#[doc(hidden)]
pub mod linguist;
#[cfg(feature = "pronom")]
#[doc(hidden)]
pub mod pronom;
#[cfg(feature = "wikidata")]
#[doc(hidden)]
pub mod wikidata;

/// Returns an iterator over all enabled file formats.
#[doc(hidden)]
pub fn file_formats() -> impl Iterator<Item = &'static FileFormat> {
    let chained = default::FILE_FORMATS.iter().copied();
    #[cfg(feature = "custom")]
    let chained = chained.chain(custom::FILE_FORMATS.iter().copied());
    #[cfg(feature = "httpd")]
    let chained = chained.chain(httpd::FILE_FORMATS.iter().copied());
    #[cfg(feature = "iana")]
    let chained = chained.chain(iana::FILE_FORMATS.iter().copied());
    #[cfg(feature = "linguist")]
    let chained = chained.chain(linguist::FILE_FORMATS.iter().copied());
    #[cfg(feature = "pronom")]
    let chained = chained.chain(pronom::FILE_FORMATS.iter().copied());
    #[cfg(feature = "wikidata")]
    let chained = chained.chain(wikidata::FILE_FORMATS.iter().copied());

    chained
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_formats() {
        let file_formats: Vec<&FileFormat> = file_formats().collect();
        assert!(!file_formats.is_empty());
    }
}
