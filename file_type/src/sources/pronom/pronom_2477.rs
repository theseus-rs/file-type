use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2477: FileFormat = FileFormat {
    id: 2_477,
    source_type: SourceType::Pronom,
    name: "Bayesian Interchange Format File",
    extensions: &["bif"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
