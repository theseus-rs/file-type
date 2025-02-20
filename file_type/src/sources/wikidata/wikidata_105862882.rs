use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862882: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_882,
        source_type: SourceType::Wikidata,
        name: "MilkShape 3D model (ASCII)",
        extensions: &["txt"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x2F, 0x2F, 0x20, 0x4D, 0x69, 0x6C, 0x6B, 0x53, 0x68, 0x61, 0x70, 0x65,
                        0x20, 0x33, 0x44, 0x20, 0x41, 0x53, 0x43, 0x49, 0x49,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
