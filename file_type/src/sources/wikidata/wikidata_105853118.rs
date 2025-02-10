use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105853118: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_118,
        source_type: SourceType::Wikidata,
        name: "Session Manager Firefox Backup",
        extensions: &["session"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x53, 0x65, 0x73, 0x73, 0x69, 0x6F, 0x6E, 0x4D, 0x61, 0x6E, 0x61,
                        0x67, 0x65, 0x72, 0x20, 0x76, 0x32, 0x5D, 0x0D, 0x0A, 0x6E, 0x61, 0x6D,
                        0x65, 0x3D, 0x5B, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
