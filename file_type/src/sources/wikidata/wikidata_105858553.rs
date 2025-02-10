use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858553: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_553,
        source_type: SourceType::Wikidata,
        name: "Electone Registrations",
        extensions: &["b00"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xF0, 0x43, 0x70, 0x78, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
