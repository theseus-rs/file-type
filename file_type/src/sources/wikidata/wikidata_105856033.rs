use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856033: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_033,
        source_type: SourceType::Wikidata,
        name: "SPECCTRA Design",
        extensions: &["dsn"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x28])],
                },
            }],
        }],
        related_formats: &[],
    },
};
