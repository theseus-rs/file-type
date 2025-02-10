use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105865199: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_199,
        source_type: SourceType::Wikidata,
        name: "ClickArt Personal Publisher document",
        extensions: &["pub"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xAA, 0xAA, 0xAA, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
