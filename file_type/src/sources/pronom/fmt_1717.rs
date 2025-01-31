use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1717: FileFormat = FileFormat {
    id: 2_553,
    puid: "fmt/1717",
    name: "Time Stamp Token",
    extensions: &["tst"],
    media_types: &["application/vnd.etsi.timestamp-token"],
    internal_signatures: &[],
    related_formats: &[],
};
