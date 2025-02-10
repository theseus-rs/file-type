use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_61963251: FileType = FileType {
    file_format: &FileFormat {
        id: 61_963_251,
        source_type: SourceType::Wikidata,
        name: "Internet Data Query File",
        extensions: &["idq"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
