use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855911: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_911,
        source_type: SourceType::Wikidata,
        name: "Dart hypertext",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x6F, 0x6D, 0x70, 0x72, 0x65, 0x73, 0x73, 0x65, 0x64, 0x20, 0x66,
                        0x69, 0x6C, 0x65, 0x2E, 0x20, 0x55, 0x73, 0x65, 0x20, 0x44, 0x61, 0x72,
                        0x74, 0x20, 0x74, 0x6F, 0x20, 0x76, 0x69, 0x65, 0x77, 0x2E, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
