use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1341: FileFormat = FileFormat {
    id: 2_159,
    puid: "fmt/1341",
    name: "Associated Signature Container Simple (ASiC-S)",
    extensions: &["asics", "scs"],
    media_types: &["application/vnd.etsi.asic-s+zip"],
    internal_signatures: &[],
    related_formats: &[],
};
