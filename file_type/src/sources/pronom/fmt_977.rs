use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_977: FileFormat = FileFormat {
    id: 1_782,
    puid: "fmt/977",
    name: "AutoCAD Design Web Format(DWFx)",
    extensions: &["dwfx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        id: 1_456,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
