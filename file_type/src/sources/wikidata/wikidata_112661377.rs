use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_112661377: FileFormat = FileFormat {
    id: 112_661_377,
    puid: "wikidata/112661377",
    name: "VIZ Material XML Import",
    extensions: &["xml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
