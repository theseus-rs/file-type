use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1510: FileFormat = FileFormat {
    id: 2_333,
    puid: "fmt/1510",
    name: "Microsoft Visio Drawing",
    extensions: &["vsd", "vst", "vss", "vsw"],
    media_types: &["application/vnd.visio"],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        id: 1_229,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
