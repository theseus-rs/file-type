use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856371: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_371,
        source_type: SourceType::Wikidata,
        name: "DIET compressed data (generic)",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xB4, 0x4C, 0xCD, 0x21, 0x9D, 0x89])],
                },
            }],
        }],
        related_formats: &[],
    },
};
