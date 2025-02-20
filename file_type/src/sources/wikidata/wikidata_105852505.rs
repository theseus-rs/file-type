use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852505: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_505,
        source_type: SourceType::Wikidata,
        name: "Superbase Program (var 1)",
        extensions: &["sbp"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x42, 0x50, 0x0D, 0x0A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
