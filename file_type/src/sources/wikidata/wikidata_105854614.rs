use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105854614: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_614,
        source_type: SourceType::Wikidata,
        name: "AvaaBook e-book",
        extensions: &["ava"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x56, 0x41, 0x41, 0x42, 0x4F, 0x4F, 0x4B,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
