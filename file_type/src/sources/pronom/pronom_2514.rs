use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2514: FileFormat = FileFormat {
    id: 2_514,
    source_type: SourceType::Pronom,
    name: "MATLAB Script File",
    extensions: &["m"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
