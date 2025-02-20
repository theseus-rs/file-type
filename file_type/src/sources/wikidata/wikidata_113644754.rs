use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_113644754: FileType = FileType {
    file_format: &FileFormat {
        id: 113_644_754,
        source_type: SourceType::Wikidata,
        name: "Hayes JT FAX",
        extensions: &["001"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
