use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853910: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_910,
        source_type: SourceType::Wikidata,
        name: "Lizard compressed data",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x06, 0x22, 0x4D, 0x18])],
                },
            }],
        }],
        related_formats: &[],
    },
};
