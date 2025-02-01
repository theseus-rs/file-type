use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_239: FileFormat = FileFormat {
    id: 969,
    puid: "fmt/239",
    name: "Microsoft Office Binder Wizard for Windows",
    extensions: &["obz"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
