use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_123693374: FileFormat = FileFormat {
    id: 123_693_374,
    source_type: SourceType::Wikidata,
    name: "Pascal unit file",
    extensions: &["pas"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
