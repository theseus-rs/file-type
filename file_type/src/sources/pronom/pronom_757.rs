use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_757: FileFormat = FileFormat {
    id: 757,
    source_type: SourceType::Pronom,
    name: "StarOffice Writer",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
