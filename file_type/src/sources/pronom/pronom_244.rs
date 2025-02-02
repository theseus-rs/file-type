use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_244: FileFormat = FileFormat {
    id: 244,
    source_type: SourceType::Pronom,
    name: "Inset Systems Bitmap",
    extensions: &["pix"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
