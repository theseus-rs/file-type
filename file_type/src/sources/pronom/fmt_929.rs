use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_929: FileFormat = FileFormat {
    id: 1_734,
    puid: "fmt/929",
    name: "Microsoft Visio Macro-Enabled Template",
    extensions: &["vstm"],
    media_types: &["application/vnd.ms-visio.template.macroEnabled.main+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
