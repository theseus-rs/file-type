use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853316: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_316,
        source_type: SourceType::Wikidata,
        name: "SNNS result",
        extensions: &["res"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x4E, 0x4E, 0x53, 0x20, 0x72, 0x65, 0x73, 0x75, 0x6C, 0x74, 0x20,
                        0x66, 0x69, 0x6C, 0x65, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
