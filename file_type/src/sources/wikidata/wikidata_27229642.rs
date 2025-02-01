use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27229642: FileFormat = FileFormat {
    id: 27_229_642,
    puid: "wikidata/27229642",
    name: "Portable Network Graphics, version 1.2",
    extensions: &["png", "png"],
    media_types: &["image/png", "image/png"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D,
                            0x49, 0x48, 0x44, 0x52,
                        ]),
                        Token::AnyWildcard,
                        Token::Literal(&[0x69, 0x54, 0x58, 0x74]),
                    ],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
