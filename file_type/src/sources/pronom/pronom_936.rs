use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_936: FileFormat = FileFormat {
    id: 936,
    source_type: SourceType::Pronom,
    name: "Statistica Report File",
    extensions: &["str"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
