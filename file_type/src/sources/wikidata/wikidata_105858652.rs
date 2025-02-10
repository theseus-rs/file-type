use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858652: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_652,
        source_type: SourceType::Wikidata,
        name: "Agfa/Matrix SCODL bitmap",
        extensions: &["scd"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xE0, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
