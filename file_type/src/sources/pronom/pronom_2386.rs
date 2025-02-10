use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2386: FileType = FileType {
    file_format: &FileFormat {
        id: 2_386,
        source_type: SourceType::Pronom,
        name: "SpritePad Image Format",
        extensions: &["spd"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x50, 0x44])],
                },
            }],
        }],
        related_formats: &[],
    },
};
