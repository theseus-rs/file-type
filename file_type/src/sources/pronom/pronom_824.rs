use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_824: FileFormat = FileFormat {
    id: 824,
    source_type: SourceType::Pronom,
    name: "EBCDIC-US",
    extensions: &["ebcdic"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
