use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1310: FileFormat = FileFormat {
    id: 1_310,
    source_type: SourceType::Pronom,
    name: "Macro enabled Microsoft Word Document OOXML",
    extensions: &["docm"],
    media_types: &["application/vnd.ms-word.document.macroEnabled.12"],
    internal_signatures: &[],
    related_formats: &[],
};
