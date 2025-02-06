use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2567: FileFormat = FileFormat {
    id: 2_567,
    source_type: SourceType::Pronom,
    name: "Wordcraft Chapter Files",
    extensions: &["001"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
