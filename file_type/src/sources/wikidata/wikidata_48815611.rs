use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_48815611: FileFormat = FileFormat {
    id: 48_815_611,
    source_type: SourceType::Wikidata,
    name: "Framework Database, version 4",
    extensions: &["fw", "fw4"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[],
};
