use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855427: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_427,
        source_type: SourceType::Wikidata,
        name: "Fast Find document List",
        extensions: &["ffl"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x46, 0x4C, 0x31, 0x2E, 0x30])],
                },
            }],
        }],
        related_formats: &[],
    },
};
