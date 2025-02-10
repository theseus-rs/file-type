use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105863925: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_925,
        source_type: SourceType::Wikidata,
        name: "PC-Type Macro",
        extensions: &["mac"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x43, 0x2D, 0x54, 0x59, 0x50, 0x45, 0x20, 0x4D, 0x61, 0x63, 0x72,
                        0x6F, 0x73, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
