use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2355: FileFormat = FileFormat {
    id: 2_355,
    source_type: SourceType::Pronom,
    name: "Serif PagePlus Publication",
    extensions: &["ppp", "ppb", "ppx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::HasPriorityOver,
        id: 1_470,
    }],
};
