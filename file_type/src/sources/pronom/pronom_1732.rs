use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1732: FileFormat = FileFormat {
    id: 1_732,
    source_type: SourceType::Pronom,
    name: "Microsoft Visio Macro-Enabled Drawing",
    extensions: &["vsdm"],
    media_types: &["application/vnd.ms-visio.drawing.macroEnabled.main+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
