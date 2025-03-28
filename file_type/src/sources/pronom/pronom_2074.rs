use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2074: FileType = FileType {
    file_format: &FileFormat {
        id: 2_074,
        source_type: SourceType::Pronom,
        name: "MapInfo Workspace File",
        extensions: &["wor"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x21, 0x57, 0x6F, 0x72, 0x6B, 0x73, 0x70, 0x61, 0x63, 0x65,
                        ]),
                        Token::WildcardCountRange(0, 6),
                        Token::Literal(&[0x21, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E]),
                        Token::WildcardCountRange(0, 6),
                        Token::Literal(&[0x21, 0x43, 0x68, 0x61, 0x72, 0x73, 0x65, 0x74]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
