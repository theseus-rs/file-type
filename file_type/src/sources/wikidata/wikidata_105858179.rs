use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858179: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_179,
        source_type: SourceType::Wikidata,
        name: "EasyCrypto encrypted",
        extensions: &["encrypted"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x54, 0x53, 0x02, 0x00, 0x01, 0x45, 0x6E, 0x63, 0x72, 0x79, 0x70, 0x74,
                        0x65, 0x64, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x2E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
