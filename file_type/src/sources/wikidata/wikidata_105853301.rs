use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105853301: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_301,
        source_type: SourceType::Wikidata,
        name: "3M Printscape document (v1.x)",
        extensions: &["std"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x56, 0x00, 0xFF])],
                },
            }],
        }],
        related_formats: &[],
    },
};
