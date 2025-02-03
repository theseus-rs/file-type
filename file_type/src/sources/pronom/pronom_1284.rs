use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1284: FileFormat = FileFormat {
    id: 1_284,
    source_type: SourceType::Pronom,
    name: "Wireless Bitmap",
    extensions: &["wbmp"],
    media_types: &["image/vnd-wap-wbmp"],
    internal_signatures: &[],
    related_formats: &[],
};
