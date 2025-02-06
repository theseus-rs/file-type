use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2159: FileFormat = FileFormat {
    id: 2_159,
    source_type: SourceType::Pronom,
    name: "Associated Signature Container Simple (ASiC-S)",
    extensions: &["asics", "scs"],
    media_types: &["application/vnd.etsi.asic-s+zip"],
    signatures: &[],
    related_formats: &[],
};
