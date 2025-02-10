use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_44069213: FileType = FileType {
    file_format: &FileFormat {
        id: 44_069_213,
        source_type: SourceType::Wikidata,
        name: "STATA Data File Format, version 104",
        extensions: &["dta"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
