use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29904540: FileFormat = FileFormat {
    id: 29_904_540,
    source_type: SourceType::Wikidata,
    name: "Statistical Analysis System log file",
    extensions: &["log"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
