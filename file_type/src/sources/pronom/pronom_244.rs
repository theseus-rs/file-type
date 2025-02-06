use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_244: FileFormat = FileFormat {
    id: 244,
    source_type: SourceType::Pronom,
    name: "Inset Systems Bitmap",
    extensions: &["pix"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
