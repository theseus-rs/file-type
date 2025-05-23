use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862200: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_200,
        source_type: SourceType::Wikidata,
        name: "Half-life Model",
        extensions: &["mdl"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x49, 0x44, 0x53, 0x54])],
                },
            }],
        }],
        related_formats: &[],
    },
};
