use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854858: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_858,
        source_type: SourceType::Wikidata,
        name: "Aldus LZW compressed data",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x4C, 0x44, 0x55, 0x53, 0x20, 0x4C, 0x5A, 0x57, 0x20, 0x20, 0x20,
                        0x31, 0x2E, 0x30, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
