use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_856: FileFormat = FileFormat {
    id: 856,
    source_type: SourceType::Pronom,
    name: "form*Z Project File",
    extensions: &["fmz"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
