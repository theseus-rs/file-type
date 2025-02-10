use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105849909: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_909,
        source_type: SourceType::Wikidata,
        name: "Corel Color Palette",
        extensions: &["cpl"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xDC, 0xDC])],
                },
            }],
        }],
        related_formats: &[],
    },
};
