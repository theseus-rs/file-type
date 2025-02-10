use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105863893: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_893,
        source_type: SourceType::Wikidata,
        name: "Navigon map",
        extensions: &["map"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x11, 0xFE, 0xD5, 0x0B, 0x05, 0x00, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
