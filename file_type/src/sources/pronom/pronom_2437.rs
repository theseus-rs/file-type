use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2437: FileType = FileType {
    file_format: &FileFormat {
        id: 2_437,
        source_type: SourceType::Pronom,
        name: "Viacom New Media Graphics",
        extensions: &["vnm", "000"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x56, 0x4E, 0x4D, 0x1A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
