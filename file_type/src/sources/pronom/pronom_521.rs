use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_521: FileFormat = FileFormat {
    id: 521,
    source_type: SourceType::Pronom,
    name: "SAS Data File",
    extensions: &["ssd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
