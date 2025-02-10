use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105854178: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_178,
        source_type: SourceType::Wikidata,
        name: "AkAbak Script",
        extensions: &["aks"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x4B, 0x41, 0x42, 0x41, 0x4B, 0x5F, 0x53, 0x43, 0x52, 0x49, 0x50,
                        0x54, 0x5F, 0x31, 0x30, 0x30, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
