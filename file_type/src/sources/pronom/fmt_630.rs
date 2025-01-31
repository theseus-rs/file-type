use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_630: FileFormat = FileFormat {
    id: 1_429,
    puid: "fmt/630",
    name: "Microsoft PowerPoint Macro-Enabled Show",
    extensions: &["ppsm"],
    media_types: &["application/vnd.ms-powerpoint.slideshow.macroEnabled.12"],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        id: 941,
        relationship_type: RelationshipType::IsSubtypeOf,
    }],
};
