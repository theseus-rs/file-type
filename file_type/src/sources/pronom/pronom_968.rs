use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_968: FileFormat = FileFormat {
    id: 968,
    source_type: SourceType::Pronom,
    name: "Microsoft Office Binder Template for Windows",
    extensions: &["obt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
