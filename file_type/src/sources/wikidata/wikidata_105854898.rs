use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105854898: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_898,
        source_type: SourceType::Wikidata,
        name: "Android adb backup (unencrypted)",
        extensions: &["ab"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x4E, 0x44, 0x52, 0x4F, 0x49, 0x44, 0x20, 0x42, 0x41, 0x43, 0x4B,
                        0x55, 0x50, 0x0A, 0x31, 0x0A, 0x31, 0x0A, 0x6E, 0x6F, 0x6E, 0x65, 0x0A,
                        0x78, 0xDA, 0xEC,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
