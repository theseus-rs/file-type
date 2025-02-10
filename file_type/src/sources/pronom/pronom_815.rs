use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_815: FileType = FileType {
    file_format: &FileFormat {
        id: 815,
        source_type: SourceType::Pronom,
        name: "Microsoft Windows Shortcut",
        extensions: &["lnk"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4C, 0x00, 0x00, 0x00, 0x01, 0x14, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00,
                        0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
