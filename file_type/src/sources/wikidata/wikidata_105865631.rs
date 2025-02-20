use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865631: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_631,
        source_type: SourceType::Wikidata,
        name: "Preboot Execution Environment",
        extensions: &["pxe"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xEA, 0x05, 0x00, 0xC0, 0x07, 0x66, 0x9C, 0x66, 0x60, 0x0F, 0xA8, 0x0F,
                        0xA0, 0x06, 0x1E, 0x66, 0x68, 0x4C, 0x52, 0x45, 0x54, 0x2E, 0x8C, 0x16,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
