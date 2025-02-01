use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_126828176: FileFormat = FileFormat {
    id: 126_828_176,
    puid: "wikidata/126828176",
    name: "Forth source code file",
    extensions: &["fs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
