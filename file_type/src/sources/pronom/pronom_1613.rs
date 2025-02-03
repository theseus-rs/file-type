use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1613: FileFormat = FileFormat {
    id: 1_613,
    source_type: SourceType::Pronom,
    name: "StarOffice Writer",
    extensions: &["sdw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
