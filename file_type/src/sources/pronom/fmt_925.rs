use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_925: FileFormat = FileFormat {
    id: 1_730,
    puid: "fmt/925",
    name: "Microsoft Visio Stencil",
    extensions: &["vssx"],
    media_types: &["application/vnd.ms-visio.stencil.main+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
