use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105853139: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_139,
        source_type: SourceType::Wikidata,
        name: "Coda Style Sheet",
        extensions: &["sss"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x63, 0x6F, 0x6D, 0x6D, 0x65, 0x6E, 0x74, 0x20, 0x7B, 0x0A, 0x20, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
