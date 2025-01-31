use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_928: FileFormat = FileFormat {
    id: 1_733,
    puid: "fmt/928",
    name: "Microsoft Visio Macro-Enabled Stencil",
    extensions: &["vssm"],
    media_types: &["application/vnd.ms-visio.stencil.macroEnabled.main+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
