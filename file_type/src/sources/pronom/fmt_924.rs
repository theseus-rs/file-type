use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_924: FileFormat = FileFormat {
    id: 1_729,
    puid: "fmt/924",
    name: "Microsoft Visio Drawing",
    extensions: &["vsdx"],
    media_types: &["application/vnd.ms-visio.drawing.main+xml"],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        id: 1_230,
        relationship_type: RelationshipType::IsSubsequentVersionOf,
    }],
};
