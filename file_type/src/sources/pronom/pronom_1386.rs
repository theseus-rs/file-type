use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1386: FileFormat = FileFormat {
    id: 1_386,
    source_type: SourceType::Pronom,
    name: "Microsoft PhotoDraw",
    extensions: &["mix"],
    media_types: &["image/vnd.mix"],
    internal_signatures: &[],
    related_formats: &[],
};
