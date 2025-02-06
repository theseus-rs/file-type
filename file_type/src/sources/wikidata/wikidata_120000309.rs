use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_120000309: FileFormat = FileFormat {
    id: 120_000_309,
    source_type: SourceType::Wikidata,
    name: "ASAP WordPower Presentation",
    extensions: &["asp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
