use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1151: FileType = FileType {
    file_format: &FileFormat {
        id: 1_151,
        source_type: SourceType::Pronom,
        name: "SuperCalc Spreadsheet",
        extensions: &["cal"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x53, 0x75, 0x70, 0x65, 0x72, 0x43, 0x61, 0x6C, 0x63, 0x20, 0x76, 0x65,
                            0x72, 0x2E,
                        ]),
                        Token::WildcardCount(2),
                        Token::Literal(&[0x31, 0x2E]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
