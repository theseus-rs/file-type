use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_273: FileFormat = FileFormat {
    id: 273,
    source_type: SourceType::Pronom,
    name: "Turbo Debugger Keystroke Recording File",
    extensions: &["tdk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
