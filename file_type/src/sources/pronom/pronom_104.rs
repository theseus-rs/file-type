use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_104: FileFormat = FileFormat {
    id: 104,
    source_type: SourceType::Pronom,
    name: "AutoLISP File",
    extensions: &["lsp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
