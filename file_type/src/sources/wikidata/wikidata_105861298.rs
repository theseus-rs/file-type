use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861298: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_298,
        source_type: SourceType::Wikidata,
        name: "Lucas Film Data - Panel",
        extensions: &["lfd"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x41, 0x4E, 0x4C])],
                },
            }],
        }],
        related_formats: &[],
    },
};
