use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849642: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_642,
        source_type: SourceType::Wikidata,
        name: "Compact compressed data",
        extensions: &["c"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xFF, 0x1F])],
                },
            }],
        }],
        related_formats: &[],
    },
};
