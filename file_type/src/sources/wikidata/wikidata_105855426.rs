use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855426: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_426,
        source_type: SourceType::Wikidata,
        name: "Frinika project",
        extensions: &["frinika"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xAC, 0xED, 0x00, 0x05, 0x73, 0x72, 0x00, 0x24, 0x63, 0x6F, 0x6D, 0x2E,
                        0x66, 0x72, 0x69, 0x6E, 0x69, 0x6B, 0x61, 0x2E, 0x70, 0x72, 0x6F, 0x6A,
                        0x65, 0x63, 0x74, 0x2E, 0x50, 0x72, 0x6F, 0x6A, 0x65, 0x63, 0x74, 0x43,
                        0x6F, 0x6E, 0x74, 0x61, 0x69, 0x6E, 0x65, 0x72,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
