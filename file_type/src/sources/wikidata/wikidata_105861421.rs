use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105861421: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_421,
        source_type: SourceType::Wikidata,
        name: "Lotus Vector Font",
        extensions: &["lvf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4C, 0x4F, 0x54, 0x55, 0x53, 0x20, 0x46, 0x4F, 0x4E, 0x54, 0x01, 0x00,
                        0x01, 0x00, 0x4F, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
