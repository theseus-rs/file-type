use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_180: FileFormat = FileFormat {
    id: 180,
    source_type: SourceType::Pronom,
    name: "Microsoft Excel Toolbar",
    extensions: &["xlb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
