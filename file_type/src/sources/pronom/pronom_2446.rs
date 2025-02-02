use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2446: FileFormat = FileFormat {
    id: 2_446,
    source_type: SourceType::Pronom,
    name: "Pascal Source Code",
    extensions: &["pas"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
