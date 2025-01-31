use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_632: FileFormat = FileFormat {
    id: 1_431,
    puid: "fmt/632",
    name: "Microsoft PowerPoint Macro-Enabled Template",
    extensions: &["potm"],
    media_types: &["application/vnd.ms-powerpoint.template.macroEnabled.12"],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        id: 941,
        relationship_type: RelationshipType::IsSubtypeOf,
    }],
};
