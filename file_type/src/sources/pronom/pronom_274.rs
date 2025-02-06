use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_274: FileFormat = FileFormat {
    id: 274,
    source_type: SourceType::Pronom,
    name: "PageMaker Time Stamp File",
    extensions: &["tym"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
