use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855769: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_769,
        source_type: SourceType::Wikidata,
        name: "DoubleDisk compressed volume (v2.5)",
        extensions: &["000"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xEB, 0x6E, 0x90, 0x49, 0x42, 0x4D, 0x4C, 0x32, 0x2E, 0x35, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
