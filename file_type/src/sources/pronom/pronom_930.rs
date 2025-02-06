use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_930: FileFormat = FileFormat {
    id: 930,
    source_type: SourceType::Pronom,
    name: "RealVideo Clip",
    extensions: &["rv"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
