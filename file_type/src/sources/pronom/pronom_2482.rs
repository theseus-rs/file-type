use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2482: FileFormat = FileFormat {
    id: 2_482,
    source_type: SourceType::Pronom,
    name: "cdrLabel Label File",
    extensions: &["clb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
