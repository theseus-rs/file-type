use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860074: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_074,
        source_type: SourceType::Wikidata,
        name: "Id Software RoQ video",
        extensions: &["roq"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x84, 0x10, 0xFF, 0xFF, 0xFF, 0xFF, 0x1E, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
