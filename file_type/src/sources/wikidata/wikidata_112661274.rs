use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_112661274: FileType = FileType {
    file_format: &FileFormat {
        id: 112_661_274,
        source_type: SourceType::Wikidata,
        name: "Lightscape Solution file",
        extensions: &["ls"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
