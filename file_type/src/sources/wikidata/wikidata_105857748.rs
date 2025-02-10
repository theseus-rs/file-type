use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857748: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_748,
        source_type: SourceType::Wikidata,
        name: "IMP eBook (v1.0)",
        extensions: &["imp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x01, 0x00, 0x42, 0x4F, 0x4F, 0x4B, 0x44, 0x4F, 0x55, 0x47,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
