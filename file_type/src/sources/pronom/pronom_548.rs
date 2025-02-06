use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_548: FileFormat = FileFormat {
    id: 548,
    source_type: SourceType::Pronom,
    name: "XYWrite Document",
    extensions: &["xyp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
