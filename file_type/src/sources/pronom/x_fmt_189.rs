use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_189: FileFormat = FileFormat {
    id: 262,
    puid: "x-fmt/189",
    name: "SDSC Image Tool Run-Length Encoded Bitmap",
    extensions: &["rle"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
