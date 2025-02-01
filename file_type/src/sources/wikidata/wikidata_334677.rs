use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_334677: FileFormat = FileFormat {
    id: 334_677,
    puid: "wikidata/334677",
    name: "DjVu",
    extensions: &["djv", "djv", "djvu", "djvu"],
    media_types: &[
        "image/vnd.djvu",
        "image/vnd.djvu",
        "image/x-djvu",
        "image/x-djvu",
    ],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x54, 0x26, 0x54, 0x46, 0x4F, 0x52, 0x4D,
                    ])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x54, 0x26, 0x54, 0x46, 0x4F, 0x52, 0x4D,
                    ])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x54, 0x26, 0x54, 0x46, 0x4F, 0x52, 0x4D,
                    ])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x54, 0x26, 0x54, 0x46, 0x4F, 0x52, 0x4D,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
