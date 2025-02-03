use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_248: FileFormat = FileFormat {
    id: 248,
    source_type: SourceType::Pronom,
    name: "MacPaint Graphics",
    extensions: &["pnt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
