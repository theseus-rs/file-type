use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_237: FileFormat = FileFormat {
    id: 967,
    puid: "fmt/237",
    name: "Microsoft Office Binder File for Windows",
    extensions: &["obd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
