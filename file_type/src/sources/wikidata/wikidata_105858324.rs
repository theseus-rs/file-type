use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858324: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_324,
        source_type: SourceType::Wikidata,
        name: "ERwin model",
        extensions: &["erwin"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xFF, 0x03, 0x00, 0x00, 0x00, 0x47, 0x44, 0x4D, 0xF8,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
