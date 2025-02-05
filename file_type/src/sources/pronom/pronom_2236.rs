use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2236: FileFormat = FileFormat {
    id: 2_236,
    source_type: SourceType::Pronom,
    name: "Corel Print House Document",
    extensions: &["cph", "cpd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
