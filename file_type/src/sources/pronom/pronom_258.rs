use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_258: FileFormat = FileFormat {
    id: 258,
    source_type: SourceType::Pronom,
    name: "Raw Bitmap",
    extensions: &["raw"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
