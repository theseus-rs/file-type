use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_398: FileFormat = FileFormat {
    id: 1_146,
    puid: "fmt/398",
    name: "Enigma Transportable File (Finale)",
    extensions: &["etf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x45, 0x4E, 0x49, 0x47, 0x4D, 0x41, 0x20]),
                    Token::Any(&[
                        &[Token::Literal(&[
                            0x54, 0x52, 0x41, 0x4E, 0x53, 0x50, 0x4F, 0x52, 0x54, 0x41, 0x42, 0x4C,
                            0x45, 0x20, 0x46, 0x49, 0x4C, 0x45,
                        ])],
                        &[Token::Literal(&[
                            0x74, 0x72, 0x61, 0x6E, 0x73, 0x70, 0x6F, 0x72, 0x74, 0x61, 0x62, 0x6C,
                            0x65, 0x20, 0x66, 0x69, 0x6C, 0x65,
                        ])],
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
