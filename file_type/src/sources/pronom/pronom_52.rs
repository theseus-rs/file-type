use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_52: FileFormat = FileFormat {
    id: 52,
    source_type: SourceType::Pronom,
    name: "Microsoft Excel Backup",
    extensions: &["xlk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
