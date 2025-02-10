use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850859: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_859,
        source_type: SourceType::Wikidata,
        name: "Kurzweil K2-serie sample",
        extensions: &["kr1", "krz"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x52, 0x41, 0x4D, 0x00, 0x00, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
