use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_124970543: FileFormat = FileFormat {
    id: 124_970_543,
    source_type: SourceType::Wikidata,
    name: "MIX message data file",
    extensions: &["mix"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
