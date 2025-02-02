use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_245: FileFormat = FileFormat {
    id: 245,
    source_type: SourceType::Pronom,
    name: "Microsoft FoxPro Library",
    extensions: &["plb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
