use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_620: FileFormat = FileFormat {
    id: 1_416,
    puid: "fmt/620",
    name: "GeoGebra",
    extensions: &["ggb"],
    media_types: &["application/vnd.geogebra.file"],
    internal_signatures: &[],
    related_formats: &[
        RelatedFormat {
            id: 1_417,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 1_413,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
