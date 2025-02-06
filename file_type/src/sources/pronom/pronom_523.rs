use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_523: FileFormat = FileFormat {
    id: 523,
    source_type: SourceType::Pronom,
    name: "Scanstudio 16-Colour Bitmap",
    extensions: &["adc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
