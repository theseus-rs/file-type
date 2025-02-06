use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_968: FileFormat = FileFormat {
    id: 968,
    source_type: SourceType::Pronom,
    name: "Microsoft Office Binder Template for Windows",
    extensions: &["obt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
