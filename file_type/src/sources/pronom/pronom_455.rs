use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_455: FileFormat = FileFormat {
    id: 455,
    source_type: SourceType::Pronom,
    name: "ACBM Graphics",
    extensions: &["acb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
