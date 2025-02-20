use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_51370239: FileType = FileType {
    file_format: &FileFormat {
        id: 51_370_239,
        source_type: SourceType::Wikidata,
        name: "Pocket Word Document",
        extensions: &["psw", "pwd"],
        media_types: &["application/x-pocket-word"],
        signatures: &[
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x7B, 0x5C, 0x70, 0x77, 0x69])],
                    },
                }],
            },
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x7B, 0x5C, 0x70, 0x77, 0x64, 0x32])],
                    },
                }],
            },
        ],
        related_formats: &[],
    },
};
