use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858367: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_367,
        source_type: SourceType::Wikidata,
        name: "ExamView Online Test",
        extensions: &["eot"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x1A, 0x45, 0x56, 0x4F, 0x4E, 0x4C, 0x49, 0x4E, 0x45,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
