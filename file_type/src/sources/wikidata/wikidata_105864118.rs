use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105864118: FileFormat = FileFormat {
    id: 105_864_118,
    puid: "wikidata/105864118",
    name: "AUMenu Menu Definition (with rem)",
    extensions: &["mdf"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
