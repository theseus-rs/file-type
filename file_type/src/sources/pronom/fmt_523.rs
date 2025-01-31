use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_523: FileFormat = FileFormat {
    id: 1_310,
    puid: "fmt/523",
    name: "Macro enabled Microsoft Word Document OOXML",
    extensions: &["docm"],
    media_types: &["application/vnd.ms-word.document.macroEnabled.12"],
    internal_signatures: &[],
    related_formats: &[],
};
