use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1688: FileFormat = FileFormat {
    id: 1_688,
    source_type: SourceType::Pronom,
    name: "AXD HTTP Handler File",
    extensions: &["axd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
