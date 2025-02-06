use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_61: FileFormat = FileFormat {
    id: 61,
    source_type: SourceType::Pronom,
    name: "CorelDraw Template",
    extensions: &["cdt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
