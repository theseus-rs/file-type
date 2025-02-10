use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851183: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_183,
        source_type: SourceType::Wikidata,
        name: "PSX TMD 3d Model",
        extensions: &["tmd"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x00, 0x00, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
