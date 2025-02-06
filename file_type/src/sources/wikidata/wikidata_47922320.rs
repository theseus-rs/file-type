use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_47922320: FileFormat = FileFormat {
    id: 47_922_320,
    source_type: SourceType::Wikidata,
    name: "AutoLISP File",
    extensions: &["lsp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
