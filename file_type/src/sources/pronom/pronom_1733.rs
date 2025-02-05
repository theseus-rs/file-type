use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1733: FileFormat = FileFormat {
    id: 1_733,
    source_type: SourceType::Pronom,
    name: "Microsoft Visio Macro-Enabled Stencil",
    extensions: &["vssm"],
    media_types: &["application/vnd.ms-visio.stencil.macroEnabled.main+xml"],
    signatures: &[],
    related_formats: &[],
};
