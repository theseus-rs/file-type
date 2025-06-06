use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2545: FileType = FileType {
    file_format: &FileFormat {
        id: 2_545,
        source_type: SourceType::Pronom,
        name: "Persuasion Presentation Interchange File",
        extensions: &["prf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x52, 0x4F, 0x4F, 0x54]),
                        Token::WildcardCount(4),
                        Token::Literal(&[
                            0x50, 0x72, 0x65, 0x73, 0x65, 0x6E, 0x74, 0x61, 0x74, 0x69, 0x6F, 0x6E,
                            0x20, 0x49, 0x6E, 0x74, 0x65, 0x72, 0x63, 0x68, 0x61, 0x6E, 0x67, 0x65,
                            0x20, 0x46, 0x69, 0x6C, 0x65, 0x20, 0x46, 0x6F, 0x72, 0x6D, 0x61, 0x74,
                        ]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
