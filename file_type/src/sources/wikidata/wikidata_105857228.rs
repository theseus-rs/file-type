use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857228: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_228,
        source_type: SourceType::Wikidata,
        name: "HEC-HMS Basin model settings",
        extensions: &["basin"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x42, 0x61, 0x73, 0x69, 0x6E, 0x3A, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
