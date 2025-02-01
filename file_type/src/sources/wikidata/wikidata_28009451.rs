use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28009451: FileFormat = FileFormat {
    id: 28_009_451,
    puid: "wikidata/28009451",
    name: "Pokémon ROM",
    extensions: &["gba"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
