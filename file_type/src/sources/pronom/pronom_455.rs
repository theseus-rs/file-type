use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_455: FileFormat = FileFormat {
    id: 455,
    source_type: SourceType::Pronom,
    name: "ACBM Graphics",
    extensions: &["acb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
