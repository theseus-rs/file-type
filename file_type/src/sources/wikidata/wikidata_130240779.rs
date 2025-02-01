use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130240779: FileFormat = FileFormat {
    id: 130_240_779,
    puid: "wikidata/130240779",
    name: "Literate Haskell source code file",
    extensions: &["lhs"],
    media_types: &["text/x-literate-haskell"],
    internal_signatures: &[],
    related_formats: &[],
};
