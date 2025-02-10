use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855948: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_948,
        source_type: SourceType::Wikidata,
        name: "Disk Doubler compressed data",
        extensions: &["dd"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xAB, 0xCD, 0x00, 0x54])],
                },
            }],
        }],
        related_formats: &[],
    },
};
