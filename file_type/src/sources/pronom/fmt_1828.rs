use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1828: FileFormat = FileFormat {
    id: 2_679,
    puid: "fmt/1828",
    name: "XLSX Strict OOXML Spreadsheet",
    extensions: &["xlsx"],
    media_types: &["application/vnd.openxmlformats-officedocument.wordprocessingml.document"],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        id: 940,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
