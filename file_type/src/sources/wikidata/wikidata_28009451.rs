use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28009451: FileFormat = FileFormat {
    id: 28_009_451,
    source_type: SourceType::Wikidata,
    name: "Pokémon ROM",
    extensions: &["gba"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
