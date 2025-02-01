use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_62619668: FileFormat = FileFormat {
    id: 62_619_668,
    puid: "wikidata/62619668",
    name: "7-bit ANSI Text",
    extensions: &["ans"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
