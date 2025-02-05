use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_525: FileFormat = FileFormat {
    id: 525,
    source_type: SourceType::Pronom,
    name: "StarOffice Calc",
    extensions: &["sdc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
