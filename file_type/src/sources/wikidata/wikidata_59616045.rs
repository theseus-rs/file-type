use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_59616045: FileFormat = FileFormat {
    id: 59_616_045,
    source_type: SourceType::Wikidata,
    name: "Zope export file",
    extensions: &["zexp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
