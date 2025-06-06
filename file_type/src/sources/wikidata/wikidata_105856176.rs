use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856176: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_176,
        source_type: SourceType::Wikidata,
        name: "Darwin Pond File",
        extensions: &["dwp"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x2E, 0x44, 0x57, 0x50, 0x03, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                        0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
