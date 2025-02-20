use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1859: FileType = FileType {
    file_format: &FileFormat {
        id: 1_859,
        source_type: SourceType::Pronom,
        name: "AVCHD Clip Information File",
        extensions: &["cpi", "clpi"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x48, 0x44, 0x4D, 0x56, 0x30, 0x31, 0x30, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
