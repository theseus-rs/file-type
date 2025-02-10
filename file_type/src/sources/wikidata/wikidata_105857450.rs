use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857450: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_450,
        source_type: SourceType::Wikidata,
        name: "4th Dimension database",
        extensions: &["4db"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x44, 0x44, 0x55, 0x56])],
                },
            }],
        }],
        related_formats: &[],
    },
};
