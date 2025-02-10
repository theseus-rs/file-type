use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851188: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_188,
        source_type: SourceType::Wikidata,
        name: ".NET Micro Framework TinyFont",
        extensions: &["tinyfnt"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xA8, 0xB0, 0x95, 0xF9, 0x02, 0x00, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
