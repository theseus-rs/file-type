use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_757: FileFormat = FileFormat {
    id: 757,
    source_type: SourceType::Pronom,
    name: "StarOffice Writer",
    extensions: &[],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
