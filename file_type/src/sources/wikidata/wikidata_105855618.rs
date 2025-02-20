use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855618: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_618,
        source_type: SourceType::Wikidata,
        name: "Openlab Raw Format",
        extensions: &["olr", "olrw"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4F, 0x4C, 0x52, 0x57])],
                },
            }],
        }],
        related_formats: &[],
    },
};
