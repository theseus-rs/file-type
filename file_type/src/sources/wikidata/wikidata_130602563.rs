use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_130602563: FileType = FileType {
    file_format: &FileFormat {
        id: 130_602_563,
        source_type: SourceType::Wikidata,
        name: "ReasonML file format",
        extensions: &["re", "rei"],
        media_types: &["text/x-reasonml"],
        signatures: &[],
        related_formats: &[],
    },
};
