use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1080: FileFormat = FileFormat {
    id: 1_080,
    source_type: SourceType::Pronom,
    name: "Dreamweaver Lock File",
    extensions: &["lck"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
