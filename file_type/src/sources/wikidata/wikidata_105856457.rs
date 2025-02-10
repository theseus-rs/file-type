use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856457: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_457,
        source_type: SourceType::Wikidata,
        name: "WarCraft III saved game",
        extensions: &["w3z"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x57, 0x61, 0x72, 0x63, 0x72, 0x61, 0x66, 0x74, 0x20, 0x49, 0x49, 0x49,
                        0x20, 0x72, 0x65, 0x63, 0x6F, 0x72, 0x64, 0x65, 0x64, 0x20, 0x67, 0x61,
                        0x6D, 0x65, 0x1A, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
