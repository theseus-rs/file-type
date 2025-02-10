use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855200: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_200,
        source_type: SourceType::Wikidata,
        name: "PreForm 3D object container",
        extensions: &["form"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xF0, 0x0D, 0xF0, 0x0D, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x01,
                        0x00, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
