use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1509: FileFormat = FileFormat {
    id: 2_332,
    puid: "fmt/1509",
    name: "Microsoft Visio Drawing",
    extensions: &["vsd", "vst", "vss"],
    media_types: &["application/vnd.visio"],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        id: 1_229,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
