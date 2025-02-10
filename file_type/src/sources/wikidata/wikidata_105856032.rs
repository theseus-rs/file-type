use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856032: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_032,
        source_type: SourceType::Wikidata,
        name: "DemoShield Demo (v5.x)",
        extensions: &["dbd"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0xF3, 0x4A, 0xF2, 0x4D, 0xF3, 0x4A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
