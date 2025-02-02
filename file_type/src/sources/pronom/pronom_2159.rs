use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2159: FileFormat = FileFormat {
    id: 2_159,
    source_type: SourceType::Pronom,
    name: "Associated Signature Container Simple (ASiC-S)",
    extensions: &["asics", "scs"],
    media_types: &["application/vnd.etsi.asic-s+zip"],
    internal_signatures: &[],
    related_formats: &[],
};
