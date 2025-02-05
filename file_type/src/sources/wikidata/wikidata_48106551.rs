use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_48106551: FileFormat = FileFormat {
    id: 48_106_551,
    source_type: SourceType::Wikidata,
    name: "DataFlex Query Tag Name",
    extensions: &["tag"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
