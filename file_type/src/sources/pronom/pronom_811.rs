use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_811: FileFormat = FileFormat {
    id: 811,
    source_type: SourceType::Pronom,
    name: "Deluxe Paint bitmap",
    extensions: &["lbm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
