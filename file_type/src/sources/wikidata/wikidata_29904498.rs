use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29904498: FileFormat = FileFormat {
    id: 29_904_498,
    puid: "wikidata/29904498",
    name: "Rayshade Heightfield",
    extensions: &["hf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
