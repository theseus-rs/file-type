use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_597: FileFormat = FileFormat {
    id: 1_389,
    puid: "fmt/597",
    name: "Microsoft Word Template",
    extensions: &["dotx"],
    media_types: &["application/vnd.openxmlformats-officedocument.wordprocessingml.template"],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        id: 1_160,
        relationship_type: RelationshipType::IsSubtypeOf,
    }],
};
