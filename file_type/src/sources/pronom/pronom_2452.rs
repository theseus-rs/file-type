use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2452: FileFormat = FileFormat {
    id: 2_452,
    source_type: SourceType::Pronom,
    name: "ESRI Colour File Format",
    extensions: &["clr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
