use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_241: FileFormat = FileFormat {
    id: 971,
    puid: "fmt/241",
    name: "Microsoft Office Binder Template for Windows",
    extensions: &["obt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
