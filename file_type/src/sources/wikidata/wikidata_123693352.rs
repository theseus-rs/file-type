use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_123693352: FileFormat = FileFormat {
    id: 123_693_352,
    source_type: SourceType::Wikidata,
    name: "C++ Builder Unit",
    extensions: &["ccp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
