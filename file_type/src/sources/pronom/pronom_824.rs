use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_824: FileFormat = FileFormat {
    id: 824,
    source_type: SourceType::Pronom,
    name: "EBCDIC-US",
    extensions: &["ebcdic"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
