use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_927: FileFormat = FileFormat {
    id: 1_732,
    puid: "fmt/927",
    name: "Microsoft Visio Macro-Enabled Drawing",
    extensions: &["vsdm"],
    media_types: &["application/vnd.ms-visio.drawing.macroEnabled.main+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
