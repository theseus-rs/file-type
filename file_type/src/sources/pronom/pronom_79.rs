use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_79: FileFormat = FileFormat {
    id: 79,
    source_type: SourceType::Pronom,
    name: "Visual Basic Macro",
    extensions: &["dvb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
