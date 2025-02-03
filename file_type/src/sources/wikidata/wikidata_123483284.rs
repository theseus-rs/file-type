use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_123483284: FileFormat = FileFormat {
    id: 123_483_284,
    source_type: SourceType::Wikidata,
    name: "Python type stub file",
    extensions: &["pyi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
