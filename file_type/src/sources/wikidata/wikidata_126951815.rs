use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_126951815: FileFormat = FileFormat {
    id: 126_951_815,
    puid: "wikidata/126951815",
    name: "Rust source code file",
    extensions: &["rs", "rs"],
    media_types: &["text/rust", "text/x-rust"],
    internal_signatures: &[],
    related_formats: &[],
};
