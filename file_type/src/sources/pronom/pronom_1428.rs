use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1428: FileFormat = FileFormat {
    id: 1_428,
    source_type: SourceType::Pronom,
    name: "Microsoft PowerPoint Show",
    extensions: &["ppsx"],
    media_types: &["application/vnd.openxmlformats-officedocument.presentationml.slideshow"],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::IsSubtypeOf,
        id: 941,
    }],
};
