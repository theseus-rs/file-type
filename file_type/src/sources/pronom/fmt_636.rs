use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_636: FileFormat = FileFormat {
    id: 1_436,
    puid: "fmt/636",
    name: "Microsoft PowerPoint Macro-Enabled Slide",
    extensions: &["sldm"],
    media_types: &["application/vnd.ms-powerpoint.slide.macroEnabled.12"],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        id: 941,
        relationship_type: RelationshipType::IsSubtypeOf,
    }],
};
