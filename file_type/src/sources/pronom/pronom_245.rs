use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_245: FileFormat = FileFormat {
    id: 245,
    source_type: SourceType::Pronom,
    name: "Microsoft FoxPro Library",
    extensions: &["plb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
