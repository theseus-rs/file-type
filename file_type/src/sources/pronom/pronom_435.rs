use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_435: FileFormat = FileFormat {
    id: 435,
    source_type: SourceType::Pronom,
    name: "IBM DisplayWrite Final Form Text File",
    extensions: &["fft"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
