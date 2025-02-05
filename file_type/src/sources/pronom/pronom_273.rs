use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_273: FileFormat = FileFormat {
    id: 273,
    source_type: SourceType::Pronom,
    name: "Turbo Debugger Keystroke Recording File",
    extensions: &["tdk"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
