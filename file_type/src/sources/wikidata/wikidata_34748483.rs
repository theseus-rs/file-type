use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_34748483: FileType = FileType {
    file_format: &FileFormat {
        id: 34_748_483,
        source_type: SourceType::Wikidata,
        name: "TD0",
        extensions: &["td0"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x54, 0x44, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
