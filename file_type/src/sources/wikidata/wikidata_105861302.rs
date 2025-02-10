use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105861302: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_302,
        source_type: SourceType::Wikidata,
        name: "Lextek Language Identification Module",
        extensions: &["lid"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4C, 0x65, 0x78, 0x74, 0x65, 0x6B, 0x20, 0x49, 0x6E, 0x74, 0x65, 0x72,
                        0x6E, 0x61, 0x74, 0x69, 0x6F, 0x6E, 0x61, 0x6C, 0x20, 0x4C, 0x61, 0x6E,
                        0x67, 0x75, 0x61, 0x67, 0x65, 0x20, 0x49, 0x64, 0x65, 0x6E, 0x74, 0x69,
                        0x66, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6F, 0x6E, 0x20, 0x4D, 0x6F, 0x64,
                        0x75, 0x6C, 0x65, 0x2E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
