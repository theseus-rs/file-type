use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860734: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_734,
        source_type: SourceType::Wikidata,
        name: "REINER SCT transfer file",
        extensions: &["rsct"],
        media_types: &["application/x-reiner-rsct"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0xA6, 0x59, 0xD5, 0x1F, 0x8B, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00,
                        0x02, 0x00, 0xEC,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
