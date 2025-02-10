use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105860735: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_735,
        source_type: SourceType::Wikidata,
        name: "RightWriter Style (v5 Win)",
        extensions: &["rwt"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x52, 0x69, 0x67, 0x68, 0x74, 0x57, 0x72, 0x69, 0x74, 0x65, 0x72, 0x20,
                        0x53, 0x74, 0x79, 0x6C, 0x65, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x20, 0x56,
                        0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x20, 0x35, 0x2E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
