use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860865: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_865,
        source_type: SourceType::Wikidata,
        name: "RDFZ game data container",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x52, 0x44, 0x46, 0x5A, 0x04, 0x00, 0x00, 0x00, 0x5A, 0x6C, 0x69, 0x62,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
