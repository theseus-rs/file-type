use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851455: FileFormat = FileFormat {
    id: 105_851_455,
    source_type: SourceType::Wikidata,
    name: "Windows 98-7 Desktop Theme (with CRLF)",
    extensions: &["the", "theme"],
    media_types: &["text/plain"],
    signatures: &[
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x0D, 0x0A])],
                },
            }],
        },
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x0D, 0x0A])],
                },
            }],
        },
    ],
    related_formats: &[],
};
