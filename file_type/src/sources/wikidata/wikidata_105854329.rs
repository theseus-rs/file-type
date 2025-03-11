use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854329: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_329,
        source_type: SourceType::Wikidata,
        name: "SimplE Lossless Audio",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x65, 0x4C, 0x61])],
                },
            }],
        }],
        related_formats: &[],
    },
};
