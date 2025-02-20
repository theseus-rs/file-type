use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862073: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_073,
        source_type: SourceType::Wikidata,
        name: "L3DT Map Group File",
        extensions: &["mgf"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4C, 0x33, 0x44, 0x54, 0x20, 0x4D, 0x61, 0x70, 0x20, 0x47, 0x72, 0x6F,
                        0x75, 0x70, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x0D, 0x0A, 0x56, 0x65, 0x72,
                        0x73, 0x69, 0x6F, 0x6E, 0x3A, 0x09,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
