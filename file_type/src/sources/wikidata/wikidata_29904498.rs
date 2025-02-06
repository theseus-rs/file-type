use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29904498: FileFormat = FileFormat {
    id: 29_904_498,
    source_type: SourceType::Wikidata,
    name: "Rayshade Heightfield",
    extensions: &["hf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
