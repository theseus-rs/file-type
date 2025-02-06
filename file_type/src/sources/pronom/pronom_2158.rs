use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2158: FileFormat = FileFormat {
    id: 2_158,
    source_type: SourceType::Pronom,
    name: "BDOC",
    extensions: &["bdoc"],
    media_types: &["application/vnd.bdoc-1.0"],
    signatures: &[],
    related_formats: &[],
};
