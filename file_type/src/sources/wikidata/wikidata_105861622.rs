use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105861622: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_622,
        source_type: SourceType::Wikidata,
        name: "LaserDRW drawing",
        extensions: &["lyz"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x2E, 0x4C, 0x59, 0x5A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
