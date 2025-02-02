use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2351: FileFormat = FileFormat {
    id: 2_351,
    source_type: SourceType::Pronom,
    name: "Serif DrawPlus Drawing",
    extensions: &["dpp", "dpa", "dpx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::HasPriorityOver,
        id: 2_352,
    }],
};
