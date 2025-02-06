use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_435: FileFormat = FileFormat {
    id: 435,
    source_type: SourceType::Pronom,
    name: "IBM DisplayWrite Final Form Text File",
    extensions: &["fft"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
