use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_972: FileFormat = FileFormat {
    id: 972,
    source_type: SourceType::Pronom,
    name: "Microsoft Office Binder Wizard for Windows",
    extensions: &["obz"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
