use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105849829: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_829,
        source_type: SourceType::Wikidata,
        name: "Sangduck Map",
        extensions: &["cmf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x61, 0x6E, 0x67, 0x64, 0x75, 0x63, 0x6B, 0x20, 0x4D, 0x61, 0x70,
                        0x20, 0x46, 0x69, 0x6C, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
