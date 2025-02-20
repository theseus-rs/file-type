use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855770: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_770,
        source_type: SourceType::Wikidata,
        name: "DAX compressed CD image",
        extensions: &["dax"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x44, 0x41, 0x58, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
