use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862973: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_973,
        source_type: SourceType::Wikidata,
        name: "Mini V preset",
        extensions: &["minibank"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x49, 0x4E, 0x49, 0x00, 0x04, 0x30, 0x30, 0x30, 0x30, 0x42, 0x41,
                        0x4E, 0x4B,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
