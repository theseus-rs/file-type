use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_238: FileFormat = FileFormat {
    id: 238,
    source_type: SourceType::Pronom,
    name: "PICS Animation",
    extensions: &["pcs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
