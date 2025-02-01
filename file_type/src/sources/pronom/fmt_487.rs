use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_487: FileFormat = FileFormat {
    id: 1_274,
    puid: "fmt/487",
    name: "Macro Enabled Microsoft Powerpoint",
    extensions: &["pptm"],
    media_types: &["application/vnd.ms-powerpoint.presentation.macroEnabled.12"],
    internal_signatures: &[],
    related_formats: &[],
};
