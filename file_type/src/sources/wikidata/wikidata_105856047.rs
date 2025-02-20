use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856047: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_047,
        source_type: SourceType::Wikidata,
        name: "Cadlogic Instinct Drawing",
        extensions: &["dp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3B, 0x20, 0x44, 0x50, 0x20, 0x76, 0x65, 0x72, 0x2E, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
