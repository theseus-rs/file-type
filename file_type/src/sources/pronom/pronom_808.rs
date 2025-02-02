use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_808: FileFormat = FileFormat {
    id: 808,
    source_type: SourceType::Pronom,
    name: "Text Configuration file",
    extensions: &["ini"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
