use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28009451: FileType = FileType {
    file_format: &FileFormat {
        id: 28_009_451,
        source_type: SourceType::Wikidata,
        name: "Pokémon ROM",
        extensions: &["gba"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
