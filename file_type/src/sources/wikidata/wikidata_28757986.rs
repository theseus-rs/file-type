use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_28757986: FileType = FileType {
    file_format: &FileFormat {
        id: 28_757_986,
        source_type: SourceType::Wikidata,
        name: "IPS manifest",
        extensions: &["mf", "p5m"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x73, 0x65, 0x74, 0x20, 0x6E, 0x61, 0x6D, 0x65, 0x3D, 0x70, 0x6B, 0x67,
                        0x2E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
