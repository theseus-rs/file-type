use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967190: FileFormat = FileFormat {
    id: 27_967_190,
    puid: "wikidata/27967190",
    name: "General Digital Music module",
    extensions: &["gdm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
