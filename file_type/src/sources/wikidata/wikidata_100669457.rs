use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_100669457: FileType = FileType {
    file_format: &FileFormat {
        id: 100_669_457,
        source_type: SourceType::Wikidata,
        name: "Apple iWork Document, version 14",
        extensions: &["iwa", "key", "numbers", "pages", "template"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
