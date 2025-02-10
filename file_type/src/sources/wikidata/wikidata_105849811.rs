use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105849811: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_811,
        source_type: SourceType::Wikidata,
        name: "Wii Color Swapping Animation",
        extensions: &["clr0"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x4C, 0x52, 0x30])],
                },
            }],
        }],
        related_formats: &[],
    },
};
