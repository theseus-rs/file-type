use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861599: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_599,
        source_type: SourceType::Wikidata,
        name: "LogicWorks circuit (v3.x)",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x55, 0x55, 0x00, 0xC8, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
