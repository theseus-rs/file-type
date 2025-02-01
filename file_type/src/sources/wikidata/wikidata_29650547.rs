use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29650547: FileFormat = FileFormat {
    id: 29_650_547,
    puid: "wikidata/29650547",
    name: "Package",
    extensions: &["pack"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
