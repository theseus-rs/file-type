use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105860401: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_401,
        source_type: SourceType::Wikidata,
        name: "DevExpress Report layout (v1)",
        extensions: &["repx"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x2F, 0x2F, 0x2F, 0x20, 0x3C, 0x58, 0x52, 0x54, 0x79, 0x70, 0x65, 0x49,
                        0x6E, 0x66, 0x6F, 0x3E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
