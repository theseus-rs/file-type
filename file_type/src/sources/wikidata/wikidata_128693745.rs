use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_128693745: FileFormat = FileFormat {
    id: 128_693_745,
    puid: "wikidata/128693745",
    name: "BNF file",
    extensions: &["bnf"],
    media_types: &["text/x-bnf"],
    internal_signatures: &[],
    related_formats: &[],
};
