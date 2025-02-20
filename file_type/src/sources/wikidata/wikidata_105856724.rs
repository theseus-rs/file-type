use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856724: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_724,
        source_type: SourceType::Wikidata,
        name: "Yamaha Tyros4 custom voice",
        extensions: &["uvd", "uvn"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x58, 0x56, 0x48, 0x44, 0x00, 0x00, 0x00, 0x50, 0x54, 0x79, 0x72, 0x6F,
                        0x73, 0x34,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
