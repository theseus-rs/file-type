use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_309440: FileType = FileType {
    file_format: &FileFormat {
        id: 309_440,
        source_type: SourceType::Wikidata,
        name: "WebM",
        extensions: &["webm"],
        media_types: &["audio/webm", "video/webm"],
        signatures: &[
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x1A, 0x45, 0xDF, 0xA3]),
                            Token::WildcardCountRange(0, 32),
                            Token::Literal(&[0x42, 0x82, 0x84, 0x77, 0x65, 0x62, 0x6D, 0x42, 0x87]),
                        ],
                    },
                }],
            },
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x1A, 0x45, 0xDF, 0xA3])],
                    },
                }],
            },
        ],
        related_formats: &[],
    },
};
