use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105859993: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_993,
        source_type: SourceType::Wikidata,
        name: "Valve Package (v1)",
        extensions: &["vpk"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x34, 0x12, 0xAA, 0x55, 0x01, 0x00, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
