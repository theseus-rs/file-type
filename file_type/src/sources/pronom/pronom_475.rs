use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_475: FileFormat = FileFormat {
    id: 475,
    source_type: SourceType::Pronom,
    name: "Dr Halo Bitmap",
    extensions: &["cut"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
