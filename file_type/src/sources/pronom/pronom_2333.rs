use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2333: FileFormat = FileFormat {
    id: 2_333,
    source_type: SourceType::Pronom,
    name: "Microsoft Visio Drawing",
    extensions: &["vsd", "vst", "vss", "vsw"],
    media_types: &["application/vnd.visio"],
    signatures: &[],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::HasPriorityOver,
        id: 1_229,
    }],
};
