use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110946240: FileFormat = FileFormat {
    id: 110_946_240,
    source_type: SourceType::Wikidata,
    name: "Drools Rule Language",
    extensions: &["drl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
