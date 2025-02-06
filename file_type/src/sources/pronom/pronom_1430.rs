use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1430: FileFormat = FileFormat {
    id: 1_430,
    source_type: SourceType::Pronom,
    name: "Microsoft PowerPoint Template",
    extensions: &["potx"],
    media_types: &["application/vnd.openxmlformats-officedocument.presentationml.template"],
    signatures: &[],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::IsSubtypeOf,
        id: 941,
    }],
};
