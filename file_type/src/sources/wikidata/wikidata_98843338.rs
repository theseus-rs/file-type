use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_98843338: FileType = FileType {
    file_format: &FileFormat {
        id: 98_843_338,
        source_type: SourceType::Wikidata,
        name: "Affinity Photo document",
        extensions: &["afphoto"],
        media_types: &["application/octet-stream"],
        signatures: &[
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x6E, 0x73, 0x72, 0x50, 0x23, 0x49, 0x6E, 0x66,
                        ])],
                    },
                }],
            },
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x00, 0xFF, 0x4B, 0x41])],
                    },
                }],
            },
        ],
        related_formats: &[],
    },
};
