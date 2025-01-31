use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_128221992: FileFormat = FileFormat {
    id: 128_221_992,
    puid: "wikidata/128221992",
    name: "Zimbu file",
    extensions: &["zu"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
