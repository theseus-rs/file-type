use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1284: FileFormat = FileFormat {
    id: 1_284,
    source_type: SourceType::Pronom,
    name: "Wireless Bitmap",
    extensions: &["wbmp"],
    media_types: &["image/vnd-wap-wbmp"],
    signatures: &[],
    related_formats: &[],
};
