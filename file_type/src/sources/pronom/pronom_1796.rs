use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1796: FileFormat = FileFormat {
    id: 1_796,
    source_type: SourceType::Pronom,
    name: "SHA256 File",
    extensions: &["sha256"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
