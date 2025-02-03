use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2178: FileFormat = FileFormat {
    id: 2_178,
    source_type: SourceType::Pronom,
    name: "Picture Publisher Bitmap",
    extensions: &["ppf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
