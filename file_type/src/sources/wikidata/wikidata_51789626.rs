use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_51789626: FileType = FileType {
    file_format: &FileFormat {
        id: 51_789_626,
        source_type: SourceType::Wikidata,
        name: "Fixed Width Values Text File",
        extensions: &[],
        media_types: &["text/plain"],
        signatures: &[],
        related_formats: &[],
    },
};
