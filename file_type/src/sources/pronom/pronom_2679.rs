use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2679: FileFormat = FileFormat {
    id: 2_679,
    source_type: SourceType::Pronom,
    name: "XLSX Strict OOXML Spreadsheet",
    extensions: &["xlsx"],
    media_types: &["application/vnd.openxmlformats-officedocument.wordprocessingml.document"],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::HasPriorityOver,
        id: 940,
    }],
};
