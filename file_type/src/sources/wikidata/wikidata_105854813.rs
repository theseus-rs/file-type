use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854813: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_813,
        source_type: SourceType::Wikidata,
        name: "DURILCA compressed file",
        extensions: &["dur"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xA4, 0x93])],
                },
            }],
        }],
        related_formats: &[],
    },
};
