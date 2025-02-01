use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_926: FileFormat = FileFormat {
    id: 1_731,
    puid: "fmt/926",
    name: "Microsoft Visio Template",
    extensions: &["vstx"],
    media_types: &["application/vnd.ms-visio.template.main+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
