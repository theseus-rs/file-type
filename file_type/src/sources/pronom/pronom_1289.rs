use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1289: FileFormat = FileFormat {
    id: 1_289,
    source_type: SourceType::Pronom,
    name: "Bentley V8 DGN",
    extensions: &["dgn"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
