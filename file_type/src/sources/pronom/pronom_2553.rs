use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2553: FileFormat = FileFormat {
    id: 2_553,
    source_type: SourceType::Pronom,
    name: "Time Stamp Token",
    extensions: &["tst"],
    media_types: &["application/vnd.etsi.timestamp-token"],
    internal_signatures: &[],
    related_formats: &[],
};
