use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854809: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_809,
        source_type: SourceType::Wikidata,
        name: "mp3HD audio",
        extensions: &["mp3"],
        media_types: &["audio/mpeg3"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x49, 0x44, 0x33, 0x03, 0x00, 0x00, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
