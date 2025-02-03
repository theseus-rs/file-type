use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_926: FileFormat = FileFormat {
    id: 926,
    source_type: SourceType::Pronom,
    name: "Mathematica Notebook",
    extensions: &["nb"],
    media_types: &["application/mathematica"],
    internal_signatures: &[],
    related_formats: &[],
};
