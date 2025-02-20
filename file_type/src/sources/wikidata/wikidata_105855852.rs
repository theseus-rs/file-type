use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855852: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_852,
        source_type: SourceType::Wikidata,
        name: "DC2N DMP format (v0)",
        extensions: &["dmp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x44, 0x43, 0x32, 0x4E, 0x2D, 0x54, 0x41, 0x50, 0x2D, 0x52, 0x41, 0x57,
                        0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
