use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_52: FileFormat = FileFormat {
    id: 52,
    source_type: SourceType::Pronom,
    name: "Microsoft Excel Backup",
    extensions: &["xlk"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
