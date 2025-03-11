use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860557: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_557,
        source_type: SourceType::Wikidata,
        name: "Relic Chunky container - game data",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x52, 0x65, 0x6C, 0x69, 0x63, 0x20, 0x43, 0x68, 0x75, 0x6E, 0x6B, 0x79,
                        0x0D, 0x0A, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
