use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_512: FileType = FileType {
    file_format: &FileFormat {
        id: 512,
        source_type: SourceType::Pronom,
        name: "Multipage Zsoft Paintbrush Bitmap Graphics",
        extensions: &["dcx"],
        media_types: &["image/x-dcx"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xB1, 0x68, 0xDE, 0x3A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
