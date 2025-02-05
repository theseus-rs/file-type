use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_213: FileFormat = FileFormat {
    id: 213,
    source_type: SourceType::Pronom,
    name: "Micrografx Designer",
    extensions: &["dsf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
