use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105865007: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_007,
        source_type: SourceType::Wikidata,
        name: "PCAnywhere32 Data",
        extensions: &["bhf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x10, 0xAB, 0xAA, 0xA8, 0x6F, 0x7B, 0x7E, 0x78, 0x7E, 0x76, 0x7E, 0x74,
                        0x77, 0x74, 0x79, 0x77,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
