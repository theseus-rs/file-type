use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2583: FileType = FileType {
    file_format: &FileFormat {
        id: 2_583,
        source_type: SourceType::Pronom,
        name: "Flow Cytometry Standard File",
        extensions: &["fcs"],
        media_types: &["application/vnd.isac.fcs"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x46, 0x43, 0x53]),
                        Token::WildcardCount(3),
                        Token::Literal(&[0x20]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
