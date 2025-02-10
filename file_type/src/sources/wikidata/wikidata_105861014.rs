use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105861014: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_014,
        source_type: SourceType::Wikidata,
        name: "LD Linker Script (with rem)",
        extensions: &["lds"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x2F, 0x2A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
