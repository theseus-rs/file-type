use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_100596946: FileFormat = FileFormat {
    id: 100_596_946,
    source_type: SourceType::Wikidata,
    name: "Minitab Worksheet, version 14",
    extensions: &["mtw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
