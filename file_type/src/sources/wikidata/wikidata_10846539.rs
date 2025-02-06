use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_10846539: FileFormat = FileFormat {
    id: 10_846_539,
    source_type: SourceType::Wikidata,
    name: "BNA",
    extensions: &["bna"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
