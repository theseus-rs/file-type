use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_72959001: FileType = FileType {
    file_format: &FileFormat {
        id: 72_959_001,
        source_type: SourceType::Wikidata,
        name: "PrintArtist project",
        extensions: &["pa"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
