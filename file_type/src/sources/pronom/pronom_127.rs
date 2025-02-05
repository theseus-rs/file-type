use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_127: FileFormat = FileFormat {
    id: 127,
    source_type: SourceType::Pronom,
    name: "Hewlett Packard Vector Graphic Plotter File",
    extensions: &["plt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
