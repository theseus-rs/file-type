use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856503: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_503,
        source_type: SourceType::Wikidata,
        name: "Wordwall data",
        extensions: &["wwf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x0F, 0x57, 0x4F, 0x52, 0x44, 0x57, 0x41, 0x4C, 0x4C, 0x44, 0x41, 0x54,
                        0x41,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
