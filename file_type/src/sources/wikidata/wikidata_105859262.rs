use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859262: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_262,
        source_type: SourceType::Wikidata,
        name: "Blockbench 3D Model",
        extensions: &["bbmodel"],
        media_types: &["text/json"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x7B, 0x22, 0x6D, 0x65, 0x74, 0x61, 0x22, 0x3A, 0x7B, 0x22, 0x66, 0x6F,
                        0x72, 0x6D, 0x61, 0x74, 0x5F, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E,
                        0x22, 0x3A, 0x22,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
