use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_123385826: FileFormat = FileFormat {
    id: 123_385_826,
    source_type: SourceType::Wikidata,
    name: "Object Animation file",
    extensions: &["can"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
