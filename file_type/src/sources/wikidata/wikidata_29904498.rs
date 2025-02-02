use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29904498: FileFormat = FileFormat {
    id: 29_904_498,
    source_type: SourceType::Wikidata,
    name: "Rayshade Heightfield",
    extensions: &["hf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
