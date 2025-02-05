use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1281: FileFormat = FileFormat {
    id: 1_281,
    source_type: SourceType::Pronom,
    name: "Microsoft Office Encrypted Document",
    extensions: &["xlsx", "pptx", "docx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
