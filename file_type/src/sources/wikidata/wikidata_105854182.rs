use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854182: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_182,
        source_type: SourceType::Wikidata,
        name: "Aegis Draw drawing",
        extensions: &[],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x38, 0x31, 0x30, 0x38, 0x36, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
