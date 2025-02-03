use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_109682753: FileFormat = FileFormat {
    id: 109_682_753,
    source_type: SourceType::Wikidata,
    name: "WinAce Archive",
    extensions: &["rar"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
