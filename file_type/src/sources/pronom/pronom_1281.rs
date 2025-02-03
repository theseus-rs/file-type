use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1281: FileFormat = FileFormat {
    id: 1_281,
    source_type: SourceType::Pronom,
    name: "Microsoft Office Encrypted Document",
    extensions: &["xlsx", "pptx", "docx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
