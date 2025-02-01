use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28009448: FileFormat = FileFormat {
    id: 28_009_448,
    puid: "wikidata/28009448",
    name: "ROM of Pokémon Mystery Dungeon Red Rescue Team",
    extensions: &["gba"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
