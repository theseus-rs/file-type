use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_258: FileFormat = FileFormat {
    id: 376,
    puid: "x-fmt/258",
    name: "Microsoft Visio Drawing",
    extensions: &["vsd", "vss", "vst"],
    media_types: &["application/vnd.visio"],
    internal_signatures: &[],
    related_formats: &[
        RelatedFormat {
            id: 767,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 1_229,
            relationship_type: RelationshipType::HasPriorityOver,
        },
    ],
};
