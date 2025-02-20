use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855933: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_933,
        source_type: SourceType::Wikidata,
        name: "DateBook Archive",
        extensions: &["dba"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x01, 0x42, 0x44])],
                },
            }],
        }],
        related_formats: &[],
    },
};
