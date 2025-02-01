use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_316: FileFormat = FileFormat {
    id: 475,
    puid: "x-fmt/316",
    name: "Dr Halo Bitmap",
    extensions: &["cut"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
