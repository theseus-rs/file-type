use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2425: FileFormat = FileFormat {
    id: 2_425,
    source_type: SourceType::Pronom,
    name: "Stata .do Command File",
    extensions: &["do"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
