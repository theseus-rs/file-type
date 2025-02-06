use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2705: FileFormat = FileFormat {
    id: 2_705,
    source_type: SourceType::Pronom,
    name: "Camtasia Recording File",
    extensions: &["camrec"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
