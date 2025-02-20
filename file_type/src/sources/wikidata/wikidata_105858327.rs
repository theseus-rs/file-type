use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858327: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_327,
        source_type: SourceType::Wikidata,
        name: "Digital Trainer Encrypted Score file",
        extensions: &["edf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x44, 0x69, 0x67, 0x69, 0x74, 0x61, 0x6C, 0x20, 0x54, 0x72, 0x61, 0x69,
                        0x6E, 0x65, 0x72, 0x20, 0x45, 0x6E, 0x63, 0x72, 0x79, 0x70, 0x74, 0x65,
                        0x64, 0x20, 0x53, 0x63, 0x6F, 0x72, 0x65, 0x20, 0x46, 0x69, 0x6C, 0x65,
                        0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
