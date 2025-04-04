use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857474: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_474,
        source_type: SourceType::Wikidata,
        name: "2Do data",
        extensions: &["2do"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x41, 0x50, 0x57, 0x2A,
                        0x72, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
