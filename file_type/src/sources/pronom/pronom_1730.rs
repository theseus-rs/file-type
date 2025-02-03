use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1730: FileFormat = FileFormat {
    id: 1_730,
    source_type: SourceType::Pronom,
    name: "Microsoft Visio Stencil",
    extensions: &["vssx"],
    media_types: &["application/vnd.ms-visio.stencil.main+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
