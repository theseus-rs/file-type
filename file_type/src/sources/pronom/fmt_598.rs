use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_598: FileFormat = FileFormat {
    id: 1_390,
    puid: "fmt/598",
    name: "Microsoft Excel Template",
    extensions: &["xltx"],
    media_types: &["application/vnd.openxmlformats-officedocument.spreadsheetml.template"],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        id: 940,
        relationship_type: RelationshipType::IsSubtypeOf,
    }],
};
