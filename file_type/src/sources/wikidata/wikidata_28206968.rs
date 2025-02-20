use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28206968: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_968,
        source_type: SourceType::Wikidata,
        name: "Photoshop brush",
        extensions: &["abr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
