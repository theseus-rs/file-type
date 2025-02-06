use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1797: FileFormat = FileFormat {
    id: 1_797,
    source_type: SourceType::Pronom,
    name: "SHA1 File",
    extensions: &["sha1"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
