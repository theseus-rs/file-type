use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_123483284: FileFormat = FileFormat {
    id: 123_483_284,
    source_type: SourceType::Wikidata,
    name: "Python type stub file",
    extensions: &["pyi"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
