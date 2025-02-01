use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_8024450: FileFormat = FileFormat {
    id: 8_024_450,
    puid: "wikidata/8024450",
    name: "Windows Script File",
    extensions: &["wsf"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
