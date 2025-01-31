use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_837: FileFormat = FileFormat {
    id: 1_638,
    puid: "fmt/837",
    name: "Quattro Pro Spreadsheet",
    extensions: &["qpw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        id: 1_637,
        relationship_type: RelationshipType::IsSubsequentVersionOf,
    }],
};
