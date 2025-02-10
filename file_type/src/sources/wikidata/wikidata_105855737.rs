use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855737: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_737,
        source_type: SourceType::Wikidata,
        name: "Rollei Digital Camera RAW picture",
        extensions: &["dcr"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x44, 0x53, 0x43, 0x2D, 0x49, 0x6D, 0x61, 0x67, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
