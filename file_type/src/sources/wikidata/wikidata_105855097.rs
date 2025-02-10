use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855097: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_097,
        source_type: SourceType::Wikidata,
        name: "777 compressed archive",
        extensions: &["777"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x37, 0x37, 0x37])],
                },
            }],
        }],
        related_formats: &[],
    },
};
