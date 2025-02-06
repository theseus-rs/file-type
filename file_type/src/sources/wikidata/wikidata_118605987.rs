use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_118605987: FileFormat = FileFormat {
    id: 118_605_987,
    source_type: SourceType::Wikidata,
    name: "Visual J# File",
    extensions: &["jsl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
