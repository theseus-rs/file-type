use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_284: FileFormat = FileFormat {
    id: 284,
    source_type: SourceType::Pronom,
    name: "Microsoft Word for Windows Macro",
    extensions: &["wpm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
