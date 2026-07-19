use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_140368155: FileType = FileType {
    file_format: &FileFormat {
        id: 140_368_155,
        source_type: SourceType::Wikidata,
        name: "AutoSketch Drawing Windows",
        extensions: &["skf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x0A, 0x00, 0x00, 0x00, 0x00, 0x0B, 0x03, 0x34,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
