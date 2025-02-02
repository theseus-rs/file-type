use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_48106551: FileFormat = FileFormat {
    id: 48_106_551,
    source_type: SourceType::Wikidata,
    name: "DataFlex Query Tag Name",
    extensions: &["tag"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
