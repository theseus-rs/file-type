use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1400: FileFormat = FileFormat {
    id: 2_218,
    puid: "fmt/1400",
    name: "Ichitaro Document",
    extensions: &["jtd", "jtt", "$td"],
    media_types: &["application/x-js-taro"],
    internal_signatures: &[],
    related_formats: &[],
};
