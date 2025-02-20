use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850113: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_113,
        source_type: SourceType::Wikidata,
        name: "DirectX Compiled Shader Object",
        extensions: &["cso"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x44, 0x58, 0x42, 0x43])],
                },
            }],
        }],
        related_formats: &[],
    },
};
