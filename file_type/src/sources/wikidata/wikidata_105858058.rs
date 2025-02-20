use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858058: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_058,
        source_type: SourceType::Wikidata,
        name: "Android Generic System Image",
        extensions: &["img"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3A, 0xFF, 0x26, 0xED, 0x01, 0x00, 0x00, 0x00, 0x1C, 0x00, 0x0C, 0x00,
                        0x00, 0x10, 0x00, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
