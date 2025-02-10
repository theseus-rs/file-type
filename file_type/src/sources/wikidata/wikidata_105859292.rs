use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105859292: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_292,
        source_type: SourceType::Wikidata,
        name: "Multipaint image (ZX)",
        extensions: &["bin"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x01, 0xFF, 0x00, 0x06, 0x0F, 0x20, 0x00, 0x18, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
