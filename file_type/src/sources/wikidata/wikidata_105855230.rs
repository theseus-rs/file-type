use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855230: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_230,
        source_type: SourceType::Wikidata,
        name: "Fattz Announcement",
        extensions: &["fza"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x46, 0x61, 0x7A, 0x7A, 0x74, 0x41, 0x6E, 0x6E, 0x6F, 0x75, 0x6E,
                        0x63, 0x65, 0x6D, 0x65, 0x6E, 0x74, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69,
                        0x6F, 0x6E, 0x3D, 0x27, 0x31, 0x2E, 0x30, 0x27, 0x3E, 0x0D, 0x0A, 0x3C,
                        0x65, 0x6E, 0x74, 0x72, 0x79, 0x20, 0x6E, 0x61, 0x6D, 0x65, 0x3D, 0x27,
                        0x45, 0x6F, 0x6E, 0x5A, 0x4E, 0x65, 0x74, 0x77, 0x6F, 0x72, 0x6B, 0x5C,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
