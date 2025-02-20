use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858605: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_605,
        source_type: SourceType::Wikidata,
        name: "Applause II Batch (v2.0)",
        extensions: &["b"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x2D, 0x54, 0x20, 0x41, 0x70, 0x70, 0x6C, 0x61, 0x75, 0x73, 0x65,
                        0x20, 0x49, 0x49, 0x20, 0x42, 0x61, 0x74, 0x63, 0x68, 0x20, 0x46, 0x69,
                        0x6C, 0x65, 0x20, 0x32, 0x2E, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
