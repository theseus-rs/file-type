use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854609: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_609,
        source_type: SourceType::Wikidata,
        name: "AptiQuiz quiz data",
        extensions: &["aq1"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x0D, 0x41, 0x70, 0x74, 0x69, 0x51, 0x75, 0x69, 0x7A, 0x20, 0x46, 0x69,
                        0x6C, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
