use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_884: FileFormat = FileFormat {
    id: 884,
    source_type: SourceType::Pronom,
    name: "Microsoft Excel for Macintosh",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
