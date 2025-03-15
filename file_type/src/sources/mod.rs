use crate::FileType;

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

/// Returns an iterator over all enabled file types.
#[doc(hidden)]
pub fn file_types() -> impl Iterator<Item = &'static FileType> {
    let chained = default::FILE_TYPES.iter().copied();
    #[cfg(feature = "httpd")]
    let chained = chained.chain(httpd::FILE_TYPES.iter().copied());
    #[cfg(feature = "iana")]
    let chained = chained.chain(iana::FILE_TYPES.iter().copied());
    #[cfg(feature = "linguist")]
    let chained = chained.chain(linguist::FILE_TYPES.iter().copied());
    #[cfg(feature = "pronom")]
    let chained = chained.chain(pronom::FILE_TYPES.iter().copied());
    #[cfg(feature = "wikidata")]
    let chained = chained.chain(wikidata::FILE_TYPES.iter().copied());

    chained
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec::Vec;

    #[test]
    fn test_file_types() {
        let file_formats: Vec<&FileType> = file_types().collect();
        assert!(!file_formats.is_empty());
    }
}
