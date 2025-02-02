use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_881: FileFormat = FileFormat {
    id: 881,
    source_type: SourceType::Pronom,
    name: "Microsoft Excel for Macintosh",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
