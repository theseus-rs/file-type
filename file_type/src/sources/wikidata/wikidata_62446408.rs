use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_62446408: FileType = FileType {
    file_format: &FileFormat {
        id: 62_446_408,
        source_type: SourceType::Wikidata,
        name: "OWL Manchester Syntax",
        extensions: &["omn"],
        media_types: &["text/owl-manchester"],
        signatures: &[],
        related_formats: &[],
    },
};
