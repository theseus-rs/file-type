use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1360: FileFormat = FileFormat {
    id: 2_178,
    puid: "fmt/1360",
    name: "Picture Publisher Bitmap",
    extensions: &["ppf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
