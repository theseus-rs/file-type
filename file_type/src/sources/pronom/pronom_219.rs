use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_219: FileFormat = FileFormat {
    id: 219,
    source_type: SourceType::Pronom,
    name: "Ventura Publisher",
    extensions: &["gen"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
