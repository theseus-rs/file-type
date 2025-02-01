use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_622: FileFormat = FileFormat {
    id: 1_418,
    puid: "fmt/622",
    name: "GeoGebra",
    extensions: &["ggb"],
    media_types: &["application/vnd.geogebra.file"],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        id: 1_417,
        relationship_type: RelationshipType::IsSubsequentVersionOf,
    }],
};
