use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_27: FileFormat = FileFormat {
    id: 27,
    source_type: SourceType::Pronom,
    name: "Revisable-Form-Text Document Content Architecture",
    extensions: &[],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
