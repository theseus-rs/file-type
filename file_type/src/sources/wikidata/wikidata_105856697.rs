use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856697: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_697,
        source_type: SourceType::Wikidata,
        name: "Universal Hint System (newer format)",
        extensions: &["uhs"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x55, 0x48, 0x53])],
                },
            }],
        }],
        related_formats: &[],
    },
};
