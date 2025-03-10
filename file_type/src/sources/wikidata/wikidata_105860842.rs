use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860842: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_842,
        source_type: SourceType::Wikidata,
        name: "RR compressed data archive",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x52, 0x52, 0x01, 0x29, 0x4B, 0x8E, 0xDE, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
