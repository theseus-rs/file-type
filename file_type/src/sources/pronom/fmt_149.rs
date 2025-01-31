use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_149: FileFormat = FileFormat {
    id: 792,
    puid: "fmt/149",
    name: "JTIP (JPEG Tiled Image Pyramid)",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
