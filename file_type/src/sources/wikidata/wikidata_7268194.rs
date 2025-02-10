use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_7268194: FileType = FileType {
    file_format: &FileFormat {
        id: 7_268_194,
        source_type: SourceType::Wikidata,
        name: "Qtch",
        extensions: &["qtch"],
        media_types: &[],
        signatures: &[
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x50, 0x71, 0x74, 0x63, 0x68, 0x00])],
                    },
                }],
            },
            Signature {
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
    },
};
