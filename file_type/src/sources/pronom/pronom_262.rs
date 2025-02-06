use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_262: FileFormat = FileFormat {
    id: 262,
    source_type: SourceType::Pronom,
    name: "SDSC Image Tool Run-Length Encoded Bitmap",
    extensions: &["rle"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
