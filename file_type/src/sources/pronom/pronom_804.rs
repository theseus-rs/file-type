use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_804: FileFormat = FileFormat {
    id: 804,
    source_type: SourceType::Pronom,
    name: "HTML Extension File",
    extensions: &["htx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
