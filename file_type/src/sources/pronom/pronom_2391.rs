use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2391: FileType = FileType {
    file_format: &FileFormat {
        id: 2_391,
        source_type: SourceType::Pronom,
        name: "ColdFusion Markup Language",
        extensions: &["cfm"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x3C, 0x63, 0x66])],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x63, 0x66]),
                            Token::WildcardCountRange(5, 100),
                            Token::Literal(&[0x3E]),
                        ],
                    },
                },
            ],
        }],
        related_formats: &[],
    },
};
