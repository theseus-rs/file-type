use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105863843: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_843,
        source_type: SourceType::Wikidata,
        name: "Multiplan for Xenix spreadsheet (v2.x)",
        extensions: &["mod"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x0A, 0xEC, 0x00, 0x00, 0x08, 0xAB, 0x08, 0x00, 0x1F, 0x00, 0x1A, 0x00,
                        0x03, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
