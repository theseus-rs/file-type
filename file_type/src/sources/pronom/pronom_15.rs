use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_15: FileFormat = FileFormat {
    id: 15,
    source_type: SourceType::Pronom,
    name: "Works for Macintosh Document",
    extensions: &[],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
