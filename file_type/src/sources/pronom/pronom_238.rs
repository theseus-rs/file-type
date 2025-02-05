use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_238: FileFormat = FileFormat {
    id: 238,
    source_type: SourceType::Pronom,
    name: "PICS Animation",
    extensions: &["pcs"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
