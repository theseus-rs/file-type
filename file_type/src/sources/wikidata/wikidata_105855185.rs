use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855185: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_185,
        source_type: SourceType::Wikidata,
        name: "Algor FEMPRO model",
        extensions: &["fem"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x45, 0x4D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
