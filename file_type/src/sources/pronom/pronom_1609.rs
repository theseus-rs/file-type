use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1609: FileFormat = FileFormat {
    id: 1_609,
    source_type: SourceType::Pronom,
    name: "StarOffice Calc",
    extensions: &["sdc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
