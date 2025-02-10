use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_114079055: FileType = FileType {
    file_format: &FileFormat {
        id: 114_079_055,
        source_type: SourceType::Wikidata,
        name: "MacBinary III",
        extensions: &["bin"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
