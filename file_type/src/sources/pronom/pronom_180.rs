use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_180: FileFormat = FileFormat {
    id: 180,
    source_type: SourceType::Pronom,
    name: "Microsoft Excel Toolbar",
    extensions: &["xlb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
