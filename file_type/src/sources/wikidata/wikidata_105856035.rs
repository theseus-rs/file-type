use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856035: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_035,
        source_type: SourceType::Wikidata,
        name: "Dynamic Env. Dynamic Paint bitmap",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x44, 0x59, 0x4E, 0x41, 0x4D, 0x49, 0x43, 0x20, 0x50, 0x41, 0x49, 0x4E,
                        0x54, 0x0D, 0x0A, 0x3A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
