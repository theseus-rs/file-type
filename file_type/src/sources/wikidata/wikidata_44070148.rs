use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_44070148: FileType = FileType {
    file_format: &FileFormat {
        id: 44_070_148,
        source_type: SourceType::Wikidata,
        name: "STATA Data File Format, version 113",
        extensions: &["dta"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
