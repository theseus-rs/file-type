use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1608: FileFormat = FileFormat {
    id: 1_608,
    source_type: SourceType::Pronom,
    name: "StarOffice Calc",
    extensions: &["sdc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
