use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859280: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_280,
        source_type: SourceType::Wikidata,
        name: "BWTC32Key encoded",
        extensions: &["b3k"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xFE, 0xFF, 0x4D, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
