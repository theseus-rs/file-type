use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_59962003: FileFormat = FileFormat {
    id: 59_962_003,
    puid: "wikidata/59962003",
    name: "ScanIt Document",
    extensions: &["sid"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
