use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_29151494: FileType = FileType {
    file_format: &FileFormat {
        id: 29_151_494,
        source_type: SourceType::Wikidata,
        name: "Quantum compressed archive",
        extensions: &["pak", "q"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x44, 0x53, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
