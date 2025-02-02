use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_548: FileFormat = FileFormat {
    id: 548,
    source_type: SourceType::Pronom,
    name: "XYWrite Document",
    extensions: &["xyp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
