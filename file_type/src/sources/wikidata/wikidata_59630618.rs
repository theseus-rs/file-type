use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_59630618: FileFormat = FileFormat {
    id: 59_630_618,
    source_type: SourceType::Wikidata,
    name: "Scriptware Script Format",
    extensions: &["sw3"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
