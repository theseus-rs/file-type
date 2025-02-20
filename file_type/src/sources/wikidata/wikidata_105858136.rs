use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858136: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_136,
        source_type: SourceType::Wikidata,
        name: "particleIllusion library",
        extensions: &["il3"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x0C, 0x49, 0x4C, 0x4C, 0x55, 0x53, 0x49, 0x4F, 0x4E, 0x33, 0x6C, 0x69,
                        0x62,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
