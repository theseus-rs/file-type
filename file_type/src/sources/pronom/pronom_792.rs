use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_792: FileFormat = FileFormat {
    id: 792,
    source_type: SourceType::Pronom,
    name: "JTIP (JPEG Tiled Image Pyramid)",
    extensions: &[],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
