use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105861273: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_273,
        source_type: SourceType::Wikidata,
        name: "Leonard Guides compiled data",
        extensions: &["lg"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4C, 0x47, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
