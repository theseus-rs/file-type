use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_34307024: FileType = FileType {
    file_format: &FileFormat {
        id: 34_307_024,
        source_type: SourceType::Wikidata,
        name: "Scratch Project Sprite",
        extensions: &["sprite"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
