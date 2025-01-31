use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1406: FileFormat = FileFormat {
    id: 2_224,
    puid: "fmt/1406",
    name: "Flow Charting",
    extensions: &["cht"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Any(&[&[Token::Literal(&[0x78])], &[Token::Literal(&[0xC8])]]),
                    Token::Literal(&[0x00, 0x78, 0x00]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
