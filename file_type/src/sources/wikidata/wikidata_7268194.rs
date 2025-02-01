use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_7268194: FileFormat = FileFormat {
    id: 7_268_194,
    puid: "wikidata/7268194",
    name: "Qtch",
    extensions: &["qtch", "qtch"],
    media_types: &[],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x71, 0x74, 0x63, 0x68, 0x00])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x00, 0x00, 0x50, 0x71, 0x74, 0x63, 0x68, 0x00,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
