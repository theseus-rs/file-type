use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858654: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_654,
        source_type: SourceType::Wikidata,
        name: "beddit compressed data (v1.0)",
        extensions: &["gz"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x62, 0x65, 0x64, 0x64, 0x69, 0x74, 0x2D, 0x63, 0x6F, 0x6D, 0x70, 0x72,
                        0x65, 0x73, 0x73, 0x65, 0x64, 0x20, 0x31, 0x2E, 0x30, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
