use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_619: FileFormat = FileFormat {
    id: 1_415,
    puid: "fmt/619",
    name: "GeoGebra",
    extensions: &["ggb"],
    media_types: &["application/vnd.geogebra.file"],
    internal_signatures: &[],
    related_formats: &[
        RelatedFormat {
            id: 1_413,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 1_414,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
