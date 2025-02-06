use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_526: FileFormat = FileFormat {
    id: 526,
    source_type: SourceType::Pronom,
    name: "StarOffice Impress",
    extensions: &["sdd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
