use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105861988: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_988,
        source_type: SourceType::Wikidata,
        name: "TommySoftware CAD/Draw drawing (v1)",
        extensions: &["mpg"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x54, 0x6F, 0x6D, 0x6D, 0x79, 0x53, 0x6F, 0x66, 0x74, 0x77, 0x61, 0x72,
                        0x65, 0x20, 0x4D, 0x50, 0x47, 0x20, 0x31, 0x2E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
