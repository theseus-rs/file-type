use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1782: FileFormat = FileFormat {
    id: 1_782,
    source_type: SourceType::Pronom,
    name: "AutoCAD Design Web Format(DWFx)",
    extensions: &["dwfx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::HasPriorityOver,
        id: 1_456,
    }],
};
