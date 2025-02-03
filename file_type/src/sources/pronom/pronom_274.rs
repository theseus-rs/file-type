use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_274: FileFormat = FileFormat {
    id: 274,
    source_type: SourceType::Pronom,
    name: "PageMaker Time Stamp File",
    extensions: &["tym"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
