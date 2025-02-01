use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105867402: FileFormat = FileFormat {
    id: 105_867_402,
    puid: "wikidata/105867402",
    name: "NSIS script (with rem)",
    extensions: &["nsi", "nsi"],
    media_types: &["text/plain", "text/x-nsis"],
    internal_signatures: &[],
    related_formats: &[],
};
