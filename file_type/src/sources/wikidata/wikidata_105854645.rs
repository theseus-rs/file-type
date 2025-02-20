use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854645: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_645,
        source_type: SourceType::Wikidata,
        name: "Absolute Database file",
        extensions: &["abs"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x42, 0x53, 0x30, 0x4C, 0x55, 0x54, 0x45, 0x44, 0x41, 0x54, 0x41,
                        0x42, 0x41, 0x53, 0x45, 0x4C,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
