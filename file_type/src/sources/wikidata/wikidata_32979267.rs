use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_32979267: FileType = FileType {
    file_format: &FileFormat {
        id: 32_979_267,
        source_type: SourceType::Wikidata,
        name: "STATA Data File Format, version 118",
        extensions: &["dta"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
