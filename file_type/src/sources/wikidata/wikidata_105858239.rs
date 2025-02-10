use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858239: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_239,
        source_type: SourceType::Wikidata,
        name: "Fashion Tracker module",
        extensions: &["ex"],
        media_types: &["audio/x-mod"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x13, 0xFC, 0x00, 0x40])],
                },
            }],
        }],
        related_formats: &[],
    },
};
