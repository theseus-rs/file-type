use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_631: FileFormat = FileFormat {
    id: 1_430,
    puid: "fmt/631",
    name: "Microsoft PowerPoint Template",
    extensions: &["potx"],
    media_types: &["application/vnd.openxmlformats-officedocument.presentationml.template"],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        id: 941,
        relationship_type: RelationshipType::IsSubtypeOf,
    }],
};
