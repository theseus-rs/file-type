use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105865344: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_344,
        source_type: SourceType::Wikidata,
        name: "Freedom Fighters graphics",
        extensions: &["paln"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4E, 0x4C, 0x41, 0x50, 0x4E, 0x4C, 0x41, 0x50,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
