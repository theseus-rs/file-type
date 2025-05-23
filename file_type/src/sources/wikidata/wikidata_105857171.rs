use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857171: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_171,
        source_type: SourceType::Wikidata,
        name: "Homeword for DeskMate",
        extensions: &["hmw"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x31, 0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x61, 0x20, 0x68,
                        0x6F, 0x6D, 0x65, 0x77, 0x6F, 0x72, 0x64, 0x20, 0x66, 0x69, 0x6C, 0x65,
                        0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x64, 0x65, 0x73, 0x6B, 0x6D, 0x61,
                        0x74, 0x65, 0x20, 0x63, 0x68, 0x61, 0x72, 0x2F, 0x61, 0x74, 0x74, 0x72,
                        0x69, 0x62, 0x75, 0x74, 0x65, 0x20, 0x70, 0x61, 0x69, 0x72, 0x73, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
