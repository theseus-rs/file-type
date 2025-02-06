use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_926: FileFormat = FileFormat {
    id: 926,
    source_type: SourceType::Pronom,
    name: "Mathematica Notebook",
    extensions: &["nb"],
    media_types: &["application/mathematica"],
    signatures: &[],
    related_formats: &[],
};
