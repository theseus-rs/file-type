use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_629: FileFormat = FileFormat {
    id: 1_428,
    puid: "fmt/629",
    name: "Microsoft PowerPoint Show",
    extensions: &["ppsx"],
    media_types: &["application/vnd.openxmlformats-officedocument.presentationml.slideshow"],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        id: 941,
        relationship_type: RelationshipType::IsSubtypeOf,
    }],
};
