use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105861356: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_356,
        source_type: SourceType::Wikidata,
        name: "lpaq1 compressed data",
        extensions: &["lpaq1"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x70, 0x51, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
