use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858560: FileFormat = FileFormat {
    id: 105_858_560,
    puid: "wikidata/105858560",
    name: "PrintFox/Pagefox bitmap (640x400)",
    extensions: &["bg", "bin"],
    media_types: &["application/octet-stream", "application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
