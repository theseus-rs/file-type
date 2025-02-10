use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28758107: FileType = FileType {
    file_format: &FileFormat {
        id: 28_758_107,
        source_type: SourceType::Wikidata,
        name: "Instant Artist GFX",
        extensions: &["gfx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
