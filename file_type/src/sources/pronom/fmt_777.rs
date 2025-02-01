use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_777: FileFormat = FileFormat {
    id: 1_576,
    puid: "fmt/777",
    name: "Microsoft Network Monitor Packet Capture",
    extensions: &["cap"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x52, 0x54, 0x53, 0x53]),
                    Token::WildcardCount(1),
                    Token::Literal(&[0x01]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
