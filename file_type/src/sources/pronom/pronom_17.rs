use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_17: FileFormat = FileFormat {
    id: 17,
    source_type: SourceType::Pronom,
    name: "FoxPro Database",
    extensions: &["dbf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
