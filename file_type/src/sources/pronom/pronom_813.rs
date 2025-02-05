use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_813: FileFormat = FileFormat {
    id: 813,
    source_type: SourceType::Pronom,
    name: "License file",
    extensions: &["lic"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
