use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1520: FileFormat = FileFormat {
    id: 1_520,
    source_type: SourceType::Pronom,
    name: "VLW Font File",
    extensions: &["vlw"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
