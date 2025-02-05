use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_109682753: FileFormat = FileFormat {
    id: 109_682_753,
    source_type: SourceType::Wikidata,
    name: "WinAce Archive",
    extensions: &["rar"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
