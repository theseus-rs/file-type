use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2215: FileFormat = FileFormat {
    id: 2_215,
    source_type: SourceType::Pronom,
    name: "FARO Laser Scan File",
    extensions: &["fls"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
