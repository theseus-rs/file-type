use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2514: FileFormat = FileFormat {
    id: 2_514,
    source_type: SourceType::Pronom,
    name: "MATLAB Script File",
    extensions: &["m"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
