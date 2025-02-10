use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105853326: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_326,
        source_type: SourceType::Wikidata,
        name: "Sonique skin",
        extensions: &["sgf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x44, 0x41, 0x46, 0x00, 0x4C, 0x49, 0x53, 0x54,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
