use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_58006953: FileType = FileType {
    file_format: &FileFormat {
        id: 58_006_953,
        source_type: SourceType::Wikidata,
        name: "TRIM Context Reference File",
        extensions: &["tr5", "txt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
