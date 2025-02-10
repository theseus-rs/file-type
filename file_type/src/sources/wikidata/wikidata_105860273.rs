use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105860273: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_273,
        source_type: SourceType::Wikidata,
        name: "Propellerhead Reason Song file",
        extensions: &["rns"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x72, 0x6F, 0x70, 0x65, 0x6C, 0x6C, 0x65, 0x72, 0x68, 0x65, 0x61,
                        0x64, 0x73, 0x20, 0x52, 0x65, 0x61, 0x73, 0x6F, 0x6E, 0x20, 0x53, 0x6F,
                        0x6E, 0x67, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
