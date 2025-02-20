use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854041: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_041,
        source_type: SourceType::Wikidata,
        name: "Power Up! Album project (v1.x)",
        extensions: &["alb"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x6F, 0x77, 0x65, 0x72, 0x55, 0x70, 0x20, 0x41, 0x6C, 0x62, 0x75,
                        0x6D, 0x20, 0x76, 0x31, 0x2E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
