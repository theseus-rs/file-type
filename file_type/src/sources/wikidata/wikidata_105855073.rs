use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855073: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_073,
        source_type: SourceType::Wikidata,
        name: "DACT compressed data",
        extensions: &["dct"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x44, 0x43, 0x54, 0xC3])],
                },
            }],
        }],
        related_formats: &[],
    },
};
