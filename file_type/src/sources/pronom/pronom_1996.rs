use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1996: FileType = FileType {
    file_format: &FileFormat {
        id: 1_996,
        source_type: SourceType::Pronom,
        name: "Dr. Halo Image Palette",
        extensions: &["pal"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x41, 0x48]),
                        Token::WildcardCount(4),
                        Token::Literal(&[0x0A, 0x00]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
