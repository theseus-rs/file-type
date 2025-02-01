use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1427: FileFormat = FileFormat {
    id: 2_245,
    puid: "fmt/1427",
    name: "MacDraw",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x44, 0x52, 0x57, 0x47]),
                    Token::Any(&[
                        &[Token::Literal(&[0x00, 0x00])],
                        &[Token::Literal(&[0x44, 0x32])],
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
