use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29000711: FileFormat = FileFormat {
    id: 29_000_711,
    source_type: SourceType::Wikidata,
    name: "TM",
    extensions: &["tm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
