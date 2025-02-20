use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852849: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_849,
        source_type: SourceType::Wikidata,
        name: "Studio 24 Song",
        extensions: &["s24"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x31, 0x25, 0x00, 0x01, 0x01, 0x00, 0x00, 0x30, 0x00, 0x18, 0x00, 0x70,
                        0x00, 0x18, 0x00, 0x00, 0x00, 0x34, 0x00, 0x40,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
