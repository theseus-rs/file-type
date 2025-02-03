use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1896: FileFormat = FileFormat {
    id: 1_896,
    source_type: SourceType::Pronom,
    name: "Visual Basic (VB) File",
    extensions: &["vb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
