use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858088: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_088,
        source_type: SourceType::Wikidata,
        name: "Infinity Engine 2-Dimensional Array (v1.0)",
        extensions: &["2da"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x32, 0x44, 0x41, 0x20, 0x56, 0x31, 0x2E, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
