use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858861: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_861,
        source_type: SourceType::Wikidata,
        name: "Brutus Application Definition",
        extensions: &["bad"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x42, 0x72, 0x75, 0x74, 0x75, 0x73, 0x20, 0x41, 0x70, 0x70, 0x6C,
                        0x69, 0x63, 0x61, 0x74, 0x69, 0x6F, 0x6E, 0x20, 0x44, 0x65, 0x66, 0x69,
                        0x6E, 0x69, 0x74, 0x69, 0x6F, 0x6E, 0x5D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
