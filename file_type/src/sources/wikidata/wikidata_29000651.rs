use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29000651: FileFormat = FileFormat {
    id: 29_000_651,
    source_type: SourceType::Wikidata,
    name: "WLD",
    extensions: &["wld"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
