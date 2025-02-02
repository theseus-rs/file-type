use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1797: FileFormat = FileFormat {
    id: 1_797,
    source_type: SourceType::Pronom,
    name: "SHA1 File",
    extensions: &["sha1"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
