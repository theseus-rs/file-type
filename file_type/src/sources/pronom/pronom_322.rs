use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_322: FileType = FileType {
    file_format: &FileFormat {
        id: 322,
        source_type: SourceType::Pronom,
        name: "MIDI Audio",
        extensions: &["mid", "midi"],
        media_types: &["audio/midi"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x4D, 0x54, 0x68, 0x64, 0x00, 0x00, 0x00, 0x06, 0x00]),
                        Token::Range(&[0x00], &[0x02]),
                        Token::WildcardCount(4),
                        Token::Literal(&[0x4D, 0x54, 0x72, 0x6B]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
