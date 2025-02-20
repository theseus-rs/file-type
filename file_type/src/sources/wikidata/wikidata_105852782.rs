use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852782: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_782,
        source_type: SourceType::Wikidata,
        name: "Spritemate",
        extensions: &["spm"],
        media_types: &["application/json"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x7B, 0x22, 0x63, 0x6F, 0x6C, 0x6F, 0x72, 0x73, 0x22, 0x3A, 0x7B, 0x22,
                        0x74, 0x22, 0x3A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
