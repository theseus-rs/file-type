use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_165: FileFormat = FileFormat {
    id: 165,
    source_type: SourceType::Pronom,
    name: "Microsoft Visio Drawing",
    extensions: &["vsd", "vst", "vss"],
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
