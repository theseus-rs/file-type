use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1479: FileFormat = FileFormat {
    id: 1_479,
    source_type: SourceType::Pronom,
    name: "Serif PagePlus Publication",
    extensions: &["ppp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::HasPriorityOver,
        id: 1_470,
    }],
};
