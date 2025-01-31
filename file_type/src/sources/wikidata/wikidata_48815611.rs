use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_48815611: FileFormat = FileFormat {
    id: 48_815_611,
    puid: "wikidata/48815611",
    name: "Framework Database, version 4",
    extensions: &["fw", "fw4"],
    media_types: &["application/octet-stream", "application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
