use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105854616: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_616,
        source_type: SourceType::Wikidata,
        name: "DS Squeeze archive",
        extensions: &["ark"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x49, 0x44, 0x46, 0x55, 0x47, 0x48, 0x54, 0x41, 0xD5, 0x02,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
