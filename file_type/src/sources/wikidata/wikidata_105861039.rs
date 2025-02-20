use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861039: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_039,
        source_type: SourceType::Wikidata,
        name: "Sony PS3 LZRC compressed data",
        extensions: &["lzrc"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x32, 0x52, 0x4C, 0x5A, 0x06])],
                },
            }],
        }],
        related_formats: &[],
    },
};
