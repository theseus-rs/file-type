use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854047: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_047,
        source_type: SourceType::Wikidata,
        name: "LZUF compressed data",
        extensions: &["lzu"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4C, 0x5A, 0x55, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
