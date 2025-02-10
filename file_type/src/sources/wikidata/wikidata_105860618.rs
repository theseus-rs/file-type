use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105860618: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_618,
        source_type: SourceType::Wikidata,
        name: "RKP game package (v1.x)",
        extensions: &["rkp"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x52, 0x4B, 0x00, 0x00, 0x56, 0x31, 0x2E])],
                },
            }],
        }],
        related_formats: &[],
    },
};
