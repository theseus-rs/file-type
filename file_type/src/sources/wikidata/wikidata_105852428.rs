use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852428: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_428,
        source_type: SourceType::Wikidata,
        name: "Sonnet Input Data",
        extensions: &["sid"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x21, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
