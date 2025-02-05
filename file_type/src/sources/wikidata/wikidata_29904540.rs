use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29904540: FileFormat = FileFormat {
    id: 29_904_540,
    source_type: SourceType::Wikidata,
    name: "Statistical Analysis System log file",
    extensions: &["log"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
