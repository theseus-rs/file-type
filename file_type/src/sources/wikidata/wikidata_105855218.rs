use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855218: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_218,
        source_type: SourceType::Wikidata,
        name: "ZODB File Storage (2.1)",
        extensions: &["fs"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x53, 0x32, 0x31, 0x03])],
                },
            }],
        }],
        related_formats: &[],
    },
};
