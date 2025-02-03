use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2058: FileFormat = FileFormat {
    id: 2_058,
    source_type: SourceType::Pronom,
    name: "Band Sequential (BSQ) Image Encoding",
    extensions: &["bsq"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
