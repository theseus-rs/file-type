use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2720: FileType = FileType {
    file_format: &FileFormat {
        id: 2_720,
        source_type: SourceType::Pronom,
        name: "Microsoft Powerpoint for Macintosh",
        extensions: &["ppt"],
        media_types: &["application/vnd.ms-PowerPoint"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x0B, 0xAD, 0xDE, 0xED, 0x00, 0x00, 0x00, 0x02,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
