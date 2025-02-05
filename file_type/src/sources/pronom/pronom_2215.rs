use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2215: FileFormat = FileFormat {
    id: 2_215,
    source_type: SourceType::Pronom,
    name: "FARO Laser Scan File",
    extensions: &["fls"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
