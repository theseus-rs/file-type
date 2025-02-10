use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105852865: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_865,
        source_type: SourceType::Wikidata,
        name: "Superbase Program (var 2)",
        extensions: &["sbp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x42, 0x50, 0x50, 0x0A, 0x2F, 0xFF, 0xC2, 0x2F,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
