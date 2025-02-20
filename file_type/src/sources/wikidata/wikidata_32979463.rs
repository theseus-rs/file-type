use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_32979463: FileType = FileType {
    file_format: &FileFormat {
        id: 32_979_463,
        source_type: SourceType::Wikidata,
        name: "STATA DTA file format, version 119",
        extensions: &["dta"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
