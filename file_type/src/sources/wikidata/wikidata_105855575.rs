use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855575: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_575,
        source_type: SourceType::Wikidata,
        name: "Oberon V4 Symbol data",
        extensions: &["sym"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xF7, 0x16])],
                },
            }],
        }],
        related_formats: &[],
    },
};
