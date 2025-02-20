use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854052: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_052,
        source_type: SourceType::Wikidata,
        name: "AAX compressed data",
        extensions: &["aax"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x40, 0xFE, 0x00, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
