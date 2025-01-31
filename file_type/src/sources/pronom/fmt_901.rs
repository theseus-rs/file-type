use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_901: FileFormat = FileFormat {
    id: 1_706,
    puid: "fmt/901",
    name: "Microsoft Works Spreadsheet",
    extensions: &["xlr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        id: 684,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
