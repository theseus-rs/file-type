use crate::format::FileFormat;
use crate::FileType;

#[cfg(feature = "custom")]
mod custom;
mod default;
#[cfg(feature = "httpd")]
pub(crate) mod httpd;
#[cfg(feature = "iana")]
pub(crate) mod iana;
#[cfg(feature = "linguist")]
pub(crate) mod linguist;
#[cfg(feature = "pronom")]
pub(crate) mod pronom;
#[cfg(feature = "wikidata")]
pub(crate) mod wikidata;

pub(crate) const FILE_FORMATS: &[&[&FileFormat]] = &[
    #[cfg(feature = "custom")]
    custom::FILE_FORMATS,
    default::FILE_FORMATS,
    #[cfg(feature = "httpd")]
    httpd::FILE_FORMATS,
    #[cfg(feature = "iana")]
    iana::FILE_FORMATS,
    #[cfg(feature = "linguist")]
    linguist::FILE_FORMATS,
    #[cfg(feature = "pronom")]
    pronom::FILE_FORMATS,
    #[cfg(feature = "wikidata")]
    wikidata::FILE_FORMATS,
];
