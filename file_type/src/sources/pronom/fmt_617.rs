use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_617: FileFormat = FileFormat {
    id: 1_413,
    puid: "fmt/617",
    name: "GeoGebra",
    extensions: &["ggb"],
    media_types: &["application/vnd.geogebra.file"],
    internal_signatures: &[],
    related_formats: &[
        RelatedFormat {
            id: 1_416,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 1_415,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
