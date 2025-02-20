use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_803: FileType = FileType {
    file_format: &FileFormat {
        id: 803,
        source_type: SourceType::Pronom,
        name: "BinHex Binary Text",
        extensions: &["hqx"],
        media_types: &["application/mac-binhex40"],
        signatures: &[Signature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[
                                0x28, 0x54, 0x68, 0x69, 0x73, 0x20, 0x66, 0x69, 0x6C, 0x65, 0x20,
                                0x6D, 0x75, 0x73, 0x74, 0x20, 0x62, 0x65, 0x20, 0x63, 0x6F, 0x6E,
                                0x76, 0x65, 0x72, 0x74, 0x65, 0x64, 0x20, 0x77, 0x69, 0x74, 0x68,
                                0x20, 0x42, 0x69, 0x6E, 0x48, 0x65, 0x78,
                            ]),
                            Token::WildcardCountRange(6, 9),
                            Token::Literal(&[0x3A]),
                        ],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(64),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x3A])],
                    },
                },
            ],
        }],
        related_formats: &[],
    },
};
