use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105861890: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_890,
        source_type: SourceType::Wikidata,
        name: "Power Translator document",
        extensions: &["mtp"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x56, 0x6D, 0x4C, 0x69, 0x73, 0x74, 0x4D, 0x44, 0x46, 0x30, 0x30,
                        0x30, 0x31, 0x0D, 0x0A, 0x7B, 0x0D, 0x0A, 0x48, 0x65, 0x61, 0x64, 0x65,
                        0x72, 0x0D, 0x0A, 0x7B, 0x0D, 0x0A, 0x0D, 0x0A, 0x30, 0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
