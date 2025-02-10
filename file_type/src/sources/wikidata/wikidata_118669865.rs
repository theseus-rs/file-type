use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_118669865: FileType = FileType {
    file_format: &FileFormat {
        id: 118_669_865,
        source_type: SourceType::Wikidata,
        name: "Manga Studio 3D Dialog file",
        extensions: &["csd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
