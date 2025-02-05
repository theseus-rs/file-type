use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_793: FileFormat = FileFormat {
    id: 793,
    source_type: SourceType::Pronom,
    name: "JPEG-LS",
    extensions: &["jls"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
