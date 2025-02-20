use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105867559: FileType = FileType {
    file_format: &FileFormat {
        id: 105_867_559,
        source_type: SourceType::Wikidata,
        name: "NVIDIA Scene Graph binary",
        extensions: &["nbf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x23, 0x4E, 0x42, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
