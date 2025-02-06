use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_334677: FileFormat = FileFormat {
    id: 334_677,
    source_type: SourceType::Wikidata,
    name: "DjVu",
    extensions: &["djv", "djvu"],
    media_types: &["image/vnd.djvu", "image/x-djvu"],
    signatures: &[
        Signature {
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
        Signature {
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
        Signature {
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
        Signature {
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
