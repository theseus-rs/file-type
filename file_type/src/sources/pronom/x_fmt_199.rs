use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_199: FileFormat = FileFormat {
    id: 273,
    puid: "x-fmt/199",
    name: "Turbo Debugger Keystroke Recording File",
    extensions: &["tdk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
