use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_48815440: FileFormat = FileFormat {
    id: 48_815_440,
    puid: "wikidata/48815440",
    name: "Framework Database, version 3",
    extensions: &["fw", "fw3"],
    media_types: &["application/octet-stream", "application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
