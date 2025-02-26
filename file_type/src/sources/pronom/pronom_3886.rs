use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_3886: FileType = FileType {
    file_format: &FileFormat {
        id: 3_886,
        source_type: SourceType::Pronom,
        name: "RawACF",
        extensions: &["rawacf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x72, 0x61, 0x64, 0x61, 0x72, 0x2E, 0x72, 0x65, 0x76, 0x69, 0x73, 0x69,
                        0x6F, 0x6E, 0x2E, 0x6D, 0x61, 0x6A, 0x6F, 0x72,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
