use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854641: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_641,
        source_type: SourceType::Wikidata,
        name: "Analysis for Windows structure",
        extensions: &["ana"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x3A, 0x5C])],
                },
            }],
        }],
        related_formats: &[],
    },
};
