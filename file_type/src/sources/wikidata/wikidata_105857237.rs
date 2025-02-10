use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857237: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_237,
        source_type: SourceType::Wikidata,
        name: "HTML Component (Unicode)",
        extensions: &["htc"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xFF, 0xFE, 0x3C, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
