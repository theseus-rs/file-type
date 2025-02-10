use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855313: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_313,
        source_type: SourceType::Wikidata,
        name: "Fluke View data",
        extensions: &["fvf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x56, 0x2E, 0x46, 0x56, 0x46, 0x1A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
