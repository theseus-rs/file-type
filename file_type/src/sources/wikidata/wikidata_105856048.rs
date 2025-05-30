use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856048: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_048,
        source_type: SourceType::Wikidata,
        name: "LocoScript Document (v1.x)",
        extensions: &["doc"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x44, 0x4F, 0x43, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
