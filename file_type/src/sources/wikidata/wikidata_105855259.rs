use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855259: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_259,
        source_type: SourceType::Wikidata,
        name: "Sound Forge project",
        extensions: &["frg"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xFF, 0xFE, 0x3C, 0x00, 0x73, 0x00, 0x6F, 0x00, 0x75, 0x00, 0x6E, 0x00,
                        0x64, 0x00, 0x5F, 0x00, 0x66, 0x00, 0x6F, 0x00, 0x72, 0x00, 0x67, 0x00,
                        0x65, 0x00, 0x5F, 0x00, 0x70, 0x00, 0x72, 0x00, 0x6F, 0x00, 0x6A, 0x00,
                        0x65, 0x00, 0x63, 0x00, 0x74, 0x00, 0x20, 0x00, 0x76, 0x00, 0x65, 0x00,
                        0x72, 0x00, 0x73, 0x00, 0x69, 0x00, 0x6F, 0x00, 0x6E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
