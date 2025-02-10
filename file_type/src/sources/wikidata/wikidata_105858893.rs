use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858893: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_893,
        source_type: SourceType::Wikidata,
        name: "Multipaint image (C64 multicolor)",
        extensions: &["bin"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x06, 0x00, 0x00, 0x0A, 0x0F, 0x28, 0x00, 0x19, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
