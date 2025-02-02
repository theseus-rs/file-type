use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_518: FileFormat = FileFormat {
    id: 518,
    source_type: SourceType::Pronom,
    name: "Professional Write Text File",
    extensions: &["pw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
