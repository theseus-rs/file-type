use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105865481: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_481,
        source_type: SourceType::Wikidata,
        name: "HOT Pop-Up Program",
        extensions: &["pgm"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xCD, 0x20, 0x03, 0x25, 0x03, 0x5D, 0xC3, 0x00, 0xE8,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
