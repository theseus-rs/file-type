use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1776: FileFormat = FileFormat {
    id: 1_776,
    source_type: SourceType::Pronom,
    name: "Microsoft Windows Movie Maker File",
    extensions: &["mswmm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
