use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_599: FileFormat = FileFormat {
    id: 1_391,
    puid: "fmt/599",
    name: "Microsoft Word Macro-Enabled Document Template",
    extensions: &["dotm"],
    media_types: &["application/vnd.ms-word.template.macroEnabled.12"],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        id: 1_160,
        relationship_type: RelationshipType::IsSubtypeOf,
    }],
};
