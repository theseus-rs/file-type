use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_127433402: FileType = FileType {
    file_format: &FileFormat {
        id: 127_433_402,
        source_type: SourceType::Wikidata,
        name: "Smalltalk Source Code",
        extensions: &["st"],
        media_types: &["text/x-smalltalk"],
        signatures: &[],
        related_formats: &[],
    },
};
