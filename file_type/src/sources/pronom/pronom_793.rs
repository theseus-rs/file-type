use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_793: FileFormat = FileFormat {
    id: 793,
    source_type: SourceType::Pronom,
    name: "JPEG-LS",
    extensions: &["jls"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
