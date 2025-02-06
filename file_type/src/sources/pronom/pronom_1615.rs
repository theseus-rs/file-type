use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1615: FileFormat = FileFormat {
    id: 1_615,
    source_type: SourceType::Pronom,
    name: "StarOffice Impress",
    extensions: &["sdd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
