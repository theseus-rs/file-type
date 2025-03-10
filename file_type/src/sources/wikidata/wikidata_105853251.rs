use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853251: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_251,
        source_type: SourceType::Wikidata,
        name: "SQ2 compressed data",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xFA, 0xFF])],
                },
            }],
        }],
        related_formats: &[],
    },
};
