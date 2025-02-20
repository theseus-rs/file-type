use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2428: FileType = FileType {
    file_format: &FileFormat {
        id: 2_428,
        source_type: SourceType::Pronom,
        name: "Type Library",
        extensions: &["tlb"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x53, 0x4C, 0x54, 0x47])],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(13),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x54, 0x59, 0x50, 0x45, 0x4C, 0x49, 0x42])],
                    },
                },
            ],
        }],
        related_formats: &[],
    },
};
