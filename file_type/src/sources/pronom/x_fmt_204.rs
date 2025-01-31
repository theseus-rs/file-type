use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_204: FileFormat = FileFormat {
    id: 284,
    puid: "x-fmt/204",
    name: "Microsoft Word for Windows Macro",
    extensions: &["wpm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
