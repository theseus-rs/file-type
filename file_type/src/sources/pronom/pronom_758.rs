use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_758: FileFormat = FileFormat {
    id: 758,
    source_type: SourceType::Pronom,
    name: "StarOffice Calc",
    extensions: &[],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
