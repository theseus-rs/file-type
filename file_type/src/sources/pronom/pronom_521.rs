use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_521: FileFormat = FileFormat {
    id: 521,
    source_type: SourceType::Pronom,
    name: "SAS Data File",
    extensions: &["ssd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
