use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1731: FileFormat = FileFormat {
    id: 1_731,
    source_type: SourceType::Pronom,
    name: "Microsoft Visio Template",
    extensions: &["vstx"],
    media_types: &["application/vnd.ms-visio.template.main+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
