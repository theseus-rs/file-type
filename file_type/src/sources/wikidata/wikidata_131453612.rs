use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131453612: FileFormat = FileFormat {
    id: 131_453_612,
    puid: "wikidata/131453612",
    name: "Zeek file format",
    extensions: &["bro", "zeek"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
