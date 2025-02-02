use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_520: FileFormat = FileFormat {
    id: 520,
    source_type: SourceType::Pronom,
    name: "SAP Document",
    extensions: &["ali"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
