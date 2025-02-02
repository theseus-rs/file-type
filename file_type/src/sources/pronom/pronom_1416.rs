use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1416: FileFormat = FileFormat {
    id: 1_416,
    source_type: SourceType::Pronom,
    name: "GeoGebra",
    extensions: &["ggb"],
    media_types: &["application/vnd.geogebra.file"],
    internal_signatures: &[],
    related_formats: &[
        RelatedFormat {
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 1_417,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 1_413,
        },
    ],
};
