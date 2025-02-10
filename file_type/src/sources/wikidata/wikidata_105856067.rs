use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856067: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_067,
        source_type: SourceType::Wikidata,
        name: "Digital Symphony relocatable module",
        extensions: &["dsym"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x02, 0x01, 0x13, 0x13, 0x14, 0x12, 0x01, 0x0B,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
