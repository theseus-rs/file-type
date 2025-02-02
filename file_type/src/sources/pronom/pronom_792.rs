use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_792: FileFormat = FileFormat {
    id: 792,
    source_type: SourceType::Pronom,
    name: "JTIP (JPEG Tiled Image Pyramid)",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
