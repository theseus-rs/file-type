use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105854750: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_750,
        source_type: SourceType::Wikidata,
        name: "Apadana Project",
        extensions: &["apr"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x4E, 0x44, 0x00, 0xA9])],
                },
            }],
        }],
        related_formats: &[],
    },
};
