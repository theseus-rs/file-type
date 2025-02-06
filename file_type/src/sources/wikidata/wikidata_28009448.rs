use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28009448: FileFormat = FileFormat {
    id: 28_009_448,
    source_type: SourceType::Wikidata,
    name: "ROM of Pokémon Mystery Dungeon Red Rescue Team",
    extensions: &["gba"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
