use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858528: FileFormat = FileFormat {
    id: 105_858_528,
    puid: "wikidata/105858528",
    name: "PrintFox/Pagefox bitmap (640x800)",
    extensions: &["bin", "pg"],
    media_types: &["application/octet-stream", "application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
