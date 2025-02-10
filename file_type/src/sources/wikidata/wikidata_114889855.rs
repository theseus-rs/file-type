use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_114889855: FileType = FileType {
    file_format: &FileFormat {
        id: 114_889_855,
        source_type: SourceType::Wikidata,
        name: "Scrapbook Factory Deluxe Effects Category file",
        extensions: &["tlx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
