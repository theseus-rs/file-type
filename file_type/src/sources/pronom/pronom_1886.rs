use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1886: FileType = FileType {
    file_format: &FileFormat {
        id: 1_886,
        source_type: SourceType::Pronom,
        name: "Microsoft Program Database",
        extensions: &["pdb"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x69, 0x63, 0x72, 0x6F, 0x73, 0x6F, 0x66, 0x74, 0x20, 0x43, 0x2F,
                        0x43, 0x2B, 0x2B, 0x20, 0x70, 0x72, 0x6F, 0x67, 0x72, 0x61, 0x6D, 0x20,
                        0x64, 0x61, 0x74, 0x61, 0x62, 0x61, 0x73, 0x65, 0x20, 0x32, 0x2E, 0x30,
                        0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
