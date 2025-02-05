use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1734: FileFormat = FileFormat {
    id: 1_734,
    source_type: SourceType::Pronom,
    name: "Microsoft Visio Macro-Enabled Template",
    extensions: &["vstm"],
    media_types: &["application/vnd.ms-visio.template.macroEnabled.main+xml"],
    signatures: &[],
    related_formats: &[],
};
