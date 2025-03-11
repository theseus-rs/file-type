use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860091: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_091,
        source_type: SourceType::Wikidata,
        name: "VENDINFO data record",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x56, 0x45, 0x4E, 0x44, 0x49, 0x4E, 0x46, 0x4F, 0x06, 0x03, 0x04, 0x05,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
