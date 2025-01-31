use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_826: FileFormat = FileFormat {
    id: 1_627,
    puid: "fmt/826",
    name: "Scriptware Script Format",
    extensions: &["sw3"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x53, 0x57, 0x33]),
                    Token::Any(&[&[Token::Literal(&[0x41])], &[Token::Literal(&[0x43])]]),
                    Token::Literal(&[
                        0x2D, 0x53, 0x63, 0x72, 0x69, 0x70, 0x74, 0x77, 0x61, 0x72, 0x65, 0x20,
                        0x62, 0x79, 0x20, 0x43, 0x69, 0x6E, 0x6F, 0x76, 0x61, 0x74, 0x69, 0x6F,
                        0x6E,
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
