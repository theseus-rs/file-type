use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855193: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_193,
        source_type: SourceType::Wikidata,
        name: "Turbo Rascal Syntax Error graphic",
        extensions: &["flf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x46, 0x4C, 0x55, 0x46, 0x46, 0x36, 0x34, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
