use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_112653466: FileFormat = FileFormat {
    id: 112_653_466,
    source_type: SourceType::Wikidata,
    name: "Professional Draw 1.0 file",
    extensions: &["pdw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
