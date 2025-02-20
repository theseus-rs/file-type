use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851449: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_449,
        source_type: SourceType::Wikidata,
        name: "Windows Desktop Theme (with CRLF)",
        extensions: &["the", "theme"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x0D, 0x0A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
