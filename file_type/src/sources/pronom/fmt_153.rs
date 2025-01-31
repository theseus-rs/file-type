use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_153: FileFormat = FileFormat {
    id: 796,
    puid: "fmt/153",
    name: "Tagged Image File Format for Image Technology (TIFF/IT)",
    extensions: &["tif", "tiff"],
    media_types: &["image/tiff"],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        id: 612,
        relationship_type: RelationshipType::IsSubtypeOf,
    }],
};
