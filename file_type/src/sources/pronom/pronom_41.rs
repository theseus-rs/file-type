use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_41: FileFormat = FileFormat {
    id: 41,
    source_type: SourceType::Pronom,
    name: "Macintosh Text File",
    extensions: &[],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
