use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_248: FileFormat = FileFormat {
    id: 248,
    source_type: SourceType::Pronom,
    name: "MacPaint Graphics",
    extensions: &["pnt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
