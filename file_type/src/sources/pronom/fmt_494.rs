use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_494: FileFormat = FileFormat {
    id: 1_281,
    puid: "fmt/494",
    name: "Microsoft Office Encrypted Document",
    extensions: &["xlsx", "pptx", "docx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
