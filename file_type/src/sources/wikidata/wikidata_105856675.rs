use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856675: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_675,
        source_type: SourceType::Wikidata,
        name: "KICK-Pascal Unit Interface",
        extensions: &["u"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4B, 0x49, 0x43, 0x4B, 0x2D, 0x50, 0x61, 0x73, 0x63, 0x61, 0x6C, 0x20,
                        0x55, 0x6E, 0x69, 0x74, 0x20, 0x49, 0x6E, 0x74, 0x65, 0x72, 0x66, 0x61,
                        0x63, 0x65, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x0A, 0x28, 0x48, 0x69, 0x6D,
                        0x70, 0x65, 0x6C, 0x73, 0x6F, 0x66, 0x74, 0x20, 0x26, 0x20, 0x4D, 0x61,
                        0x78, 0x6F, 0x6E, 0x29, 0x0A, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
