use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_123385294: FileFormat = FileFormat {
    id: 123_385_294,
    source_type: SourceType::Wikidata,
    name: "Material library file",
    extensions: &["mtl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
