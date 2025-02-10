use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858278: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_278,
        source_type: SourceType::Wikidata,
        name: "EasyC Project",
        extensions: &["ecp"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x47, 0x65, 0x6E, 0x65, 0x72, 0x61, 0x6C, 0x5D, 0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
