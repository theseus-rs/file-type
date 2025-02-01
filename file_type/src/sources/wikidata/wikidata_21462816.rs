use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_21462816: FileFormat = FileFormat {
    id: 21_462_816,
    puid: "wikidata/21462816",
    name: "Android Secure encrypted file",
    extensions: &["asec"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
