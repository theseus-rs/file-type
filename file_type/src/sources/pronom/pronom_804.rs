use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_804: FileFormat = FileFormat {
    id: 804,
    source_type: SourceType::Pronom,
    name: "HTML Extension File",
    extensions: &["htx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
