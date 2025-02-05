use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2178: FileFormat = FileFormat {
    id: 2_178,
    source_type: SourceType::Pronom,
    name: "Picture Publisher Bitmap",
    extensions: &["ppf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
