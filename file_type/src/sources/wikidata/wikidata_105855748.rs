use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855748: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_748,
        source_type: SourceType::Wikidata,
        name: "Half-Life Demo",
        extensions: &["dem"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x48, 0x4C, 0x44, 0x45, 0x4D, 0x4F])],
                },
            }],
        }],
        related_formats: &[],
    },
};
