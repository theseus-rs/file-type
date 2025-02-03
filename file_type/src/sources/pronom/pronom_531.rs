use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_531: FileFormat = FileFormat {
    id: 531,
    source_type: SourceType::Pronom,
    name: "TeX Binary File",
    extensions: &["dvi"],
    media_types: &["application/x-dvi"],
    internal_signatures: &[],
    related_formats: &[],
};
