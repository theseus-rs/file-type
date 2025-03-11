use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855736: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_736,
        source_type: SourceType::Wikidata,
        name: "DPAPI encrypted data",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x01, 0x00, 0x00, 0x00, 0xD0, 0x8C, 0x9D, 0xDF, 0x01, 0x15, 0xD1, 0x11,
                        0x8C, 0x7A, 0x00, 0xC0, 0x4F, 0xC2, 0x97, 0xEB,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
