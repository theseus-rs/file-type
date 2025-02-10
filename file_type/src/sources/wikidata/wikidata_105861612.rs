use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105861612: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_612,
        source_type: SourceType::Wikidata,
        name: "CWLS Log ASCII Standard",
        extensions: &["las"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x7E, 0x56])],
                },
            }],
        }],
        related_formats: &[],
    },
};
