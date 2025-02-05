use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858161: FileFormat = FileFormat {
    id: 105_858_161,
    source_type: SourceType::Wikidata,
    name: "Casio Graph100 ROMDISK image",
    extensions: &["cdr", "lec"],
    media_types: &["application/octet-stream"],
    signatures: &[
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xEB, 0x28, 0x90, 0x44, 0x4C, 0x52, 0x44, 0x49, 0x53, 0x4B,
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
                        0xEB, 0x28, 0x90, 0x44, 0x4C, 0x52, 0x44, 0x49, 0x53, 0x4B,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
