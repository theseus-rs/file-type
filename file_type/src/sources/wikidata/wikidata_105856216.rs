use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856216: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_216,
        source_type: SourceType::Wikidata,
        name: "DFF format (v1.0, BE)",
        extensions: &["dff"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x25, 0x44, 0x46, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
