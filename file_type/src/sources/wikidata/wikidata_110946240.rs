use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_110946240: FileFormat = FileFormat {
    id: 110_946_240,
    source_type: SourceType::Wikidata,
    name: "Drools Rule Language",
    extensions: &["drl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
