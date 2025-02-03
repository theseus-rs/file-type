use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_284: FileFormat = FileFormat {
    id: 284,
    source_type: SourceType::Pronom,
    name: "Microsoft Word for Windows Macro",
    extensions: &["wpm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
