use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_104: FileFormat = FileFormat {
    id: 104,
    source_type: SourceType::Pronom,
    name: "AutoLISP File",
    extensions: &["lsp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
