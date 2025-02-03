use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_67123981: FileFormat = FileFormat {
    id: 67_123_981,
    source_type: SourceType::Wikidata,
    name: "Print Artist craft file format",
    extensions: &["crf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
