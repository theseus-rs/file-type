use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855651: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_651,
        source_type: SourceType::Wikidata,
        name: "Oracle song",
        extensions: &["orc"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4F, 0x52, 0x41, 0x43, 0x4C, 0x45, 0x00, 0x01, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
