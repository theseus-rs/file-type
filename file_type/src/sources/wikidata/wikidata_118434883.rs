use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_118434883: FileFormat = FileFormat {
    id: 118_434_883,
    source_type: SourceType::Wikidata,
    name: "Form File",
    extensions: &["fff"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
