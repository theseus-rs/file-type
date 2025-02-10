use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1577: FileType = FileType {
    file_format: &FileFormat {
        id: 1_577,
        source_type: SourceType::Pronom,
        name: "Microsoft Network Monitor Packet Capture",
        extensions: &["cap"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x47, 0x4D, 0x42, 0x55]),
                        Token::WildcardCount(1),
                        Token::Literal(&[0x02]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
