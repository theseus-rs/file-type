use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105852737: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_737,
        source_type: SourceType::Wikidata,
        name: "Superbase form",
        extensions: &["sbv"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x45, 0x44, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
