use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105862360: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_360,
        source_type: SourceType::Wikidata,
        name: "The Player 6.1a module",
        extensions: &["p61"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x36, 0x31, 0x41])],
                },
            }],
        }],
        related_formats: &[],
    },
};
