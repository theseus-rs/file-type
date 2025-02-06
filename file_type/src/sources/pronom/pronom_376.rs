use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_376: FileFormat = FileFormat {
    id: 376,
    source_type: SourceType::Pronom,
    name: "Microsoft Visio Drawing",
    extensions: &["vsd", "vss", "vst"],
    media_types: &["application/vnd.visio"],
    signatures: &[],
    related_formats: &[
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 767,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 1_229,
        },
    ],
};
