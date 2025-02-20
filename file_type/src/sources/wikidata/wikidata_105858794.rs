use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858794: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_794,
        source_type: SourceType::Wikidata,
        name: "Multipaint image (Plus4 hi-res)",
        extensions: &["bin"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x06, 0xFF, 0x3D, 0x09, 0x78, 0x28, 0x00, 0x19, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
