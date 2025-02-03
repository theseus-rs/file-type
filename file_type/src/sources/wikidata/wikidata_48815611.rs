use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_48815611: FileFormat = FileFormat {
    id: 48_815_611,
    source_type: SourceType::Wikidata,
    name: "Framework Database, version 4",
    extensions: &["fw", "fw4"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
