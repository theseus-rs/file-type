use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858543: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_543,
        source_type: SourceType::Wikidata,
        name: "MalieGF bitmap",
        extensions: &["mgf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x61, 0x6C, 0x69, 0x65, 0x47, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
