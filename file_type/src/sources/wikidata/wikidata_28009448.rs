use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28009448: FileType = FileType {
    file_format: &FileFormat {
        id: 28_009_448,
        source_type: SourceType::Wikidata,
        name: "ROM of Pokémon Mystery Dungeon Red Rescue Team",
        extensions: &["gba"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
