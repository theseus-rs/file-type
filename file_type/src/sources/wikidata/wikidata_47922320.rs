use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47922320: FileFormat = FileFormat {
    id: 47_922_320,
    puid: "wikidata/47922320",
    name: "AutoLISP File",
    extensions: &["lsp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
