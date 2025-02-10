use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105867115: FileType = FileType {
    file_format: &FileFormat {
        id: 105_867_115,
        source_type: SourceType::Wikidata,
        name: "Nokia 3D Map",
        extensions: &["n3m"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4E, 0x33, 0x4D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
