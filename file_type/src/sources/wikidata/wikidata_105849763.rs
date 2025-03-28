use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849763: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_763,
        source_type: SourceType::Wikidata,
        name: "Satori Paint Canvas",
        extensions: &["cvs"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x30, 0x00, 0x00, 0x00, 0x47, 0x4C, 0x43, 0x57, 0x49, 0x4E, 0x44, 0x20,
                        0x70, 0x69, 0x63, 0x74, 0x75, 0x72, 0x65, 0x20, 0x66, 0x69, 0x6C, 0x65,
                        0x20, 0x28, 0x63, 0x29, 0x20, 0x53, 0x70, 0x61, 0x63, 0x65, 0x77, 0x61,
                        0x72, 0x64, 0x20, 0x47, 0x72, 0x61, 0x70, 0x68, 0x69, 0x63, 0x73, 0x20,
                        0x4C, 0x74, 0x64, 0x2E, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
