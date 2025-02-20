use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860444: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_444,
        source_type: SourceType::Wikidata,
        name: "RapidFile Data",
        extensions: &["rpd"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xE3, 0x92, 0x87, 0x45])],
                },
            }],
        }],
        related_formats: &[],
    },
};
