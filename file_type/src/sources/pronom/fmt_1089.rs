use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1089: FileFormat = FileFormat {
    id: 1_897,
    puid: "fmt/1089",
    name: "VBScript (VBS) File",
    extensions: &["vbs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
