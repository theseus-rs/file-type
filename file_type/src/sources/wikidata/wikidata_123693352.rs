use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_123693352: FileFormat = FileFormat {
    id: 123_693_352,
    source_type: SourceType::Wikidata,
    name: "C++ Builder Unit",
    extensions: &["ccp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
