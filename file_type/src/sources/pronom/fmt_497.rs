use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_497: FileFormat = FileFormat {
    id: 1_284,
    puid: "fmt/497",
    name: "Wireless Bitmap",
    extensions: &["wbmp"],
    media_types: &["image/vnd-wap-wbmp"],
    internal_signatures: &[],
    related_formats: &[],
};
