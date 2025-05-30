use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856209: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_209,
        source_type: SourceType::Wikidata,
        name: "DeleD map",
        extensions: &["dmf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x44, 0x65, 0x6C, 0x65, 0x44, 0x20, 0x4D, 0x61, 0x70, 0x20, 0x46, 0x69,
                        0x6C, 0x65, 0x3B,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
