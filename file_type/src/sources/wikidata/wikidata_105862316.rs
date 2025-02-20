use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862316: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_316,
        source_type: SourceType::Wikidata,
        name: "GMOD format module",
        extensions: &["gmod"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x47, 0x4D, 0x4F, 0x44])],
                },
            }],
        }],
        related_formats: &[],
    },
};
