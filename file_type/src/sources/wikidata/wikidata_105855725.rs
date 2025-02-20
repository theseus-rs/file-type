use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855725: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_725,
        source_type: SourceType::Wikidata,
        name: "DELTA binary dataset",
        extensions: &["dlt"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x06, 0x22, 0x44, 0x45, 0x4C, 0x54, 0x41, 0x2D, 0x44, 0x61, 0x74, 0x61,
                        0x66, 0x69, 0x6C, 0x65, 0x20, 0x28, 0x63, 0x29, 0x20, 0x47, 0x2E, 0x46,
                        0x2E, 0x20, 0x57, 0x65, 0x69, 0x6C, 0x6C, 0x65, 0x72, 0x20, 0x31, 0x39,
                        0x39, 0x36, 0x3B, 0x20, 0x43, 0x53, 0x49, 0x52, 0x4F, 0x20, 0x45, 0x6E,
                        0x74, 0x6F, 0x6D, 0x6F, 0x6C, 0x6F, 0x67, 0x79, 0x20, 0x31, 0x39, 0x39,
                        0x37, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
