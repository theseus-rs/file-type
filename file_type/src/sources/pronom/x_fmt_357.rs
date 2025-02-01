use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_357: FileFormat = FileFormat {
    id: 523,
    puid: "x-fmt/357",
    name: "Scanstudio 16-Colour Bitmap",
    extensions: &["adc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
