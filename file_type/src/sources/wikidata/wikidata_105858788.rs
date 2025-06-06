use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858788: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_788,
        source_type: SourceType::Wikidata,
        name: "Google Chrome dictionary",
        extensions: &["bdic"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x42, 0x44, 0x69, 0x63])],
                },
            }],
        }],
        related_formats: &[],
    },
};
