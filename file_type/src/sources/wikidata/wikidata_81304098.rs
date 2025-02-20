use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_81304098: FileType = FileType {
    file_format: &FileFormat {
        id: 81_304_098,
        source_type: SourceType::Wikidata,
        name: "Division dVS 3d model",
        extensions: &["biz"],
        media_types: &["application/octet-stream"],
        signatures: &[
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x62, 0x69, 0x7A, 0x20, 0x76, 0x31, 0x2E, 0x30, 0x20, 0x28, 0x63, 0x29,
                            0x20, 0x44, 0x49, 0x56, 0x49, 0x53, 0x49, 0x4F, 0x4E, 0x20, 0x4C, 0x74,
                            0x64, 0x20, 0x31, 0x39, 0x39, 0x32, 0x00,
                        ])],
                    },
                }],
            },
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x44, 0x49, 0x56, 0x2D, 0x42, 0x49, 0x5A, 0x32,
                        ])],
                    },
                }],
            },
        ],
        related_formats: &[],
    },
};
