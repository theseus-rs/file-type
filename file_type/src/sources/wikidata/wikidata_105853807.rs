use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853807: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_807,
        source_type: SourceType::Wikidata,
        name: "AniMouse Tutorial (v1.1)",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x6E, 0x69, 0x4D, 0x6F, 0x75, 0x73, 0x65, 0xAE, 0x20, 0x54, 0x75,
                        0x74, 0x6F, 0x72, 0x69, 0x61, 0x6C, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x20,
                        0x76, 0x31, 0x2E, 0x31, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
