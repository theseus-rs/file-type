use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_930: FileFormat = FileFormat {
    id: 930,
    source_type: SourceType::Pronom,
    name: "RealVideo Clip",
    extensions: &["rv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
