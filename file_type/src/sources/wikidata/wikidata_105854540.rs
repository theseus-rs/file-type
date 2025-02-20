use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854540: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_540,
        source_type: SourceType::Wikidata,
        name: "ANSI escape sequence text",
        extensions: &["ans", "asc"],
        media_types: &["text/x-ansi"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x1B, 0x5B])],
                },
            }],
        }],
        related_formats: &[],
    },
};
