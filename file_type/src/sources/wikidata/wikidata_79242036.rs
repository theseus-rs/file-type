use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_79242036: FileType = FileType {
    file_format: &FileFormat {
        id: 79_242_036,
        source_type: SourceType::Wikidata,
        name: "American College of Radiology file",
        extensions: &["acr"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x61, 0x63, 0x63, 0x3A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
