use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_621: FileFormat = FileFormat {
    id: 1_417,
    puid: "fmt/621",
    name: "GeoGebra",
    extensions: &["ggb"],
    media_types: &["application/vnd.geogebra.file"],
    internal_signatures: &[],
    related_formats: &[
        RelatedFormat {
            id: 1_418,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 1_416,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
