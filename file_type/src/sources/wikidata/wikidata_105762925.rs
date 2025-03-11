use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762925: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_925,
        source_type: SourceType::Wikidata,
        name: "XZP container format",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x58, 0x55, 0x49, 0x5A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
