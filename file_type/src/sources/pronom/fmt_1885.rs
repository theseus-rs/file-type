use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1885: FileFormat = FileFormat {
    id: 2_739,
    puid: "fmt/1885",
    name: "CloudCompare Entity File",
    extensions: &["bin"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x43, 0x42, 0x32])],
            },
        }],
    }],
    related_formats: &[],
};
