use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1613: FileFormat = FileFormat {
    id: 1_613,
    source_type: SourceType::Pronom,
    name: "StarOffice Writer",
    extensions: &["sdw"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
