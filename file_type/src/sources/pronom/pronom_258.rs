use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_258: FileFormat = FileFormat {
    id: 258,
    source_type: SourceType::Pronom,
    name: "Raw Bitmap",
    extensions: &["raw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
