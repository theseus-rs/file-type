use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_44070571: FileType = FileType {
    file_format: &FileFormat {
        id: 44_070_571,
        source_type: SourceType::Wikidata,
        name: "STATA Data File Format, version 114",
        extensions: &["dta"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
