use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_531: FileFormat = FileFormat {
    id: 531,
    source_type: SourceType::Pronom,
    name: "TeX Binary File",
    extensions: &["dvi"],
    media_types: &["application/x-dvi"],
    signatures: &[],
    related_formats: &[],
};
