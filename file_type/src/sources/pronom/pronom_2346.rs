use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2346: FileFormat = FileFormat {
    id: 2_346,
    source_type: SourceType::Pronom,
    name: "Serif DrawPlus Drawing",
    extensions: &["dpp", "dpa", "dpx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::HasPriorityOver,
        id: 2_352,
    }],
};
