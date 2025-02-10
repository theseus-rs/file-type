use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105863531: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_531,
        source_type: SourceType::Wikidata,
        name: "REBEL book Moves format",
        extensions: &["mvs"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x42, 0x6F, 0x6F, 0x6B, 0x6D, 0x6F, 0x76, 0x65, 0x73, 0x20, 0x2E, 0x2E,
                        0x2E, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
