use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105853359: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_359,
        source_type: SourceType::Wikidata,
        name: "SQL Server CE Edition database",
        extensions: &["sdf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x53, 0x53, 0x43, 0x45,
                        0x20, 0x44, 0x61, 0x74, 0x61, 0x62, 0x61, 0x73, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
