use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855392: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_392,
        source_type: SourceType::Wikidata,
        name: "netfabb Project",
        extensions: &["fabbproject"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x99, 0xE1, 0x43, 0x2A, 0x02, 0x00, 0x00, 0x00, 0x6E, 0x65, 0x74, 0x66,
                        0x61, 0x62, 0x62, 0x20, 0x50, 0x72, 0x6F, 0x6A, 0x65, 0x63, 0x74, 0x20,
                        0x46, 0x69, 0x6C, 0x65, 0x20, 0x28, 0x63, 0x29, 0x20, 0x62, 0x79, 0x20,
                        0x46, 0x49, 0x54, 0x20, 0x32, 0x30, 0x30, 0x38,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
