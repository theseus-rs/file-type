use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_44070042: FileType = FileType {
    file_format: &FileFormat {
        id: 44_070_042,
        source_type: SourceType::Wikidata,
        name: "STATA Data File Format, version 111",
        extensions: &["dta"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
