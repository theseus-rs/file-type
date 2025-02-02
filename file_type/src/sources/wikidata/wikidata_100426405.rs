use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_100426405: FileFormat = FileFormat {
    id: 100_426_405,
    source_type: SourceType::Wikidata,
    name: "Minitab Worksheet, version 13",
    extensions: &["mtw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
