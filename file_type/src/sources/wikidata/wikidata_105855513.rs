use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855513: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_513,
        source_type: SourceType::Wikidata,
        name: "InfoFile database (v1.0)",
        extensions: &["flr"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x37, 0x32, 0x32, 0x34, 0x39, 0x00, 0x00, 0x01, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
