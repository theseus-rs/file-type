use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858602: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_602,
        source_type: SourceType::Wikidata,
        name: "HSI JPEG bitmap",
        extensions: &["hsi", "jpg"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x68, 0x73, 0x69, 0x31])],
                },
            }],
        }],
        related_formats: &[],
    },
};
