use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852494: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_494,
        source_type: SourceType::Wikidata,
        name: "SimpleWriter key",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x44, 0x52, 0x4A, 0x2D, 0x53, 0x57, 0x2D, 0x33,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
