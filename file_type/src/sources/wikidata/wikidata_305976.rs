use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_305976: FileFormat = FileFormat {
    id: 305_976,
    source_type: SourceType::Wikidata,
    name: "deb",
    extensions: &["deb", "ipk", "udeb"],
    media_types: &["application/vnd.debian.binary-package"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x21, 0x3C, 0x61, 0x72, 0x63, 0x68, 0x3E, 0x0A, 0x64, 0x65, 0x62, 0x69,
                        0x61, 0x6E, 0x2D, 0x62, 0x69, 0x6E, 0x61, 0x72, 0x79,
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
                        0x21, 0x3C, 0x61, 0x72, 0x63, 0x68, 0x3E, 0x0A, 0x64, 0x65, 0x62, 0x69,
                        0x61, 0x6E, 0x2D, 0x62, 0x69, 0x6E, 0x61, 0x72, 0x79,
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
                        0x21, 0x3C, 0x61, 0x72, 0x63, 0x68, 0x3E, 0x0A, 0x64, 0x65, 0x62, 0x69,
                        0x61, 0x6E, 0x2D, 0x62, 0x69, 0x6E, 0x61, 0x72, 0x79,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
