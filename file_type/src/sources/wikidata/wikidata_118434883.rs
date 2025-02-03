use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_118434883: FileFormat = FileFormat {
    id: 118_434_883,
    source_type: SourceType::Wikidata,
    name: "Form File",
    extensions: &["fff"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
