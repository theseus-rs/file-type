use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105867576: FileType = FileType {
    file_format: &FileFormat {
        id: 105_867_576,
        source_type: SourceType::Wikidata,
        name: "DOT_MAGIC! NLQ font",
        extensions: &["nlq"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x44, 0x4F, 0x54, 0x5F, 0x4D, 0x41, 0x47, 0x49, 0x43, 0x20, 0x4E, 0x4C,
                        0x51, 0x20, 0x46, 0x4F, 0x4E, 0x54,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
