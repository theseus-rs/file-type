use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29000609: FileFormat = FileFormat {
    id: 29_000_609,
    source_type: SourceType::Wikidata,
    name: "Java Card CAP",
    extensions: &["cap"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
