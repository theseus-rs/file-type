use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855257: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_257,
        source_type: SourceType::Wikidata,
        name: "Forgotten Worlds custom music format",
        extensions: &["fw"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x57, 0x4D, 0x50])],
                },
            }],
        }],
        related_formats: &[],
    },
};
