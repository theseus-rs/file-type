use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861747: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_747,
        source_type: SourceType::Wikidata,
        name: "Exotic AdLib module",
        extensions: &["xad"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x58, 0x41, 0x44, 0x21])],
                },
            }],
        }],
        related_formats: &[],
    },
};
