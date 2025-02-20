use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860476: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_476,
        source_type: SourceType::Wikidata,
        name: "PlayStation RSD 3D model info (gen)",
        extensions: &["rsd"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x40, 0x52, 0x53, 0x44])],
                },
            }],
        }],
        related_formats: &[],
    },
};
