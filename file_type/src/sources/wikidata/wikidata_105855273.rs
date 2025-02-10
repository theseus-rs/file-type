use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855273: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_273,
        source_type: SourceType::Wikidata,
        name: "FastBack Plus setup",
        extensions: &["fb"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x42, 0x65, 0x67, 0x69, 0x6E, 0x53, 0x65, 0x74, 0x75, 0x70, 0x28, 0x22,
                        0x54, 0x72, 0x75, 0x65, 0x22, 0x29, 0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
