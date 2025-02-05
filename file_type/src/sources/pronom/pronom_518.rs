use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_518: FileFormat = FileFormat {
    id: 518,
    source_type: SourceType::Pronom,
    name: "Professional Write Text File",
    extensions: &["pw"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
