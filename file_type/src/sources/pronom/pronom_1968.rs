use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1968: FileType = FileType {
    file_format: &FileFormat {
        id: 1_968,
        source_type: SourceType::Pronom,
        name: "Folio Infobase File",
        extensions: &["nfo"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(212),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xEF, 0xCD, 0xAB, 0x89, 0x00, 0x00, 0x03, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
