use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105864947: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_947,
        source_type: SourceType::Wikidata,
        name: "CUPL PLD Program format",
        extensions: &["pld"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4E, 0x61, 0x6D, 0x65])],
                },
            }],
        }],
        related_formats: &[],
    },
};
