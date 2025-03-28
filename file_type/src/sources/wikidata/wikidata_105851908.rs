use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851908: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_908,
        source_type: SourceType::Wikidata,
        name: "sc68 soundchip music",
        extensions: &["sc68"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x43, 0x36, 0x38, 0x20, 0x4D, 0x75, 0x73, 0x69, 0x63, 0x2D, 0x66,
                        0x69, 0x6C, 0x65, 0x20, 0x2F, 0x20, 0x28, 0x63, 0x29, 0x20, 0x28, 0x42,
                        0x65, 0x4E, 0x29, 0x6A, 0x61, 0x6D, 0x69, 0x6E, 0x20, 0x47, 0x65, 0x72,
                        0x61, 0x72, 0x64, 0x20, 0x2F, 0x20, 0x53, 0x61, 0x73, 0x48, 0x69, 0x70,
                        0x41, 0x2D, 0x44, 0x65, 0x76, 0x20, 0x20, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
