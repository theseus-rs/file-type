use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130240656: FileFormat = FileFormat {
    id: 130_240_656,
    puid: "wikidata/130240656",
    name: "Literate Cryptol source code file",
    extensions: &["lcry"],
    media_types: &["text/x-literate-cryptol"],
    internal_signatures: &[],
    related_formats: &[],
};
