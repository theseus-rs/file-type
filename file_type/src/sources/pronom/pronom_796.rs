use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_796: FileFormat = FileFormat {
    id: 796,
    source_type: SourceType::Pronom,
    name: "Tagged Image File Format for Image Technology (TIFF/IT)",
    extensions: &["tif", "tiff"],
    media_types: &["image/tiff"],
    signatures: &[],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::IsSubtypeOf,
        id: 612,
    }],
};
