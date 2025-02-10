use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_64858274: FileType = FileType {
    file_format: &FileFormat {
        id: 64_858_274,
        source_type: SourceType::Wikidata,
        name: "Corel Presentations Master file format",
        extensions: &["mst"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
