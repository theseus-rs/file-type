use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853152: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_152,
        source_type: SourceType::Wikidata,
        name: "Yamaha EX5 voices format",
        extensions: &["s1v"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x59, 0x31, 0x32, 0x30, 0x30, 0x56, 0x43, 0x41, 0x20, 0x20, 0x56,
                        0x31, 0x2E, 0x30, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
