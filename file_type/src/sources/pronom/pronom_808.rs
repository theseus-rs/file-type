use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_808: FileFormat = FileFormat {
    id: 808,
    source_type: SourceType::Pronom,
    name: "Text Configuration file",
    extensions: &["ini"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
