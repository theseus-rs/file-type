use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1274: FileFormat = FileFormat {
    id: 1_274,
    source_type: SourceType::Pronom,
    name: "Macro Enabled Microsoft Powerpoint",
    extensions: &["pptm"],
    media_types: &["application/vnd.ms-powerpoint.presentation.macroEnabled.12"],
    internal_signatures: &[],
    related_formats: &[],
};
