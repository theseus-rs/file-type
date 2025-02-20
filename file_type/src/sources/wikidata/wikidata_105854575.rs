use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854575: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_575,
        source_type: SourceType::Wikidata,
        name: "Modular V preset",
        extensions: &["amb"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x4F, 0x4F, 0x47, 0x00, 0x04, 0x30, 0x30, 0x30, 0x30, 0x42, 0x41,
                        0x4E, 0x4B,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
