use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_123385339: FileFormat = FileFormat {
    id: 123_385_339,
    source_type: SourceType::Wikidata,
    name: "Object library file",
    extensions: &["obl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
