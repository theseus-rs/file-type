use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855348: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_348,
        source_type: SourceType::Wikidata,
        name: "Forge File System game data archive (v1.0)",
        extensions: &["ffs"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x09, 0x09, 0x09, 0x09, 0x46, 0x6F, 0x72, 0x67, 0x65, 0x20, 0x46, 0x69,
                        0x6C, 0x65, 0x20, 0x53, 0x79, 0x73, 0x74, 0x65, 0x6D, 0x20, 0x76, 0x31,
                        0x2E, 0x30, 0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
