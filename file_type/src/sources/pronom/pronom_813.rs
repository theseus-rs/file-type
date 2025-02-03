use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_813: FileFormat = FileFormat {
    id: 813,
    source_type: SourceType::Pronom,
    name: "License file",
    extensions: &["lic"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
