use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856782: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_782,
        source_type: SourceType::Wikidata,
        name: "Game data package",
        extensions: &["gbx"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x47, 0x42, 0x58, 0x06])],
                },
            }],
        }],
        related_formats: &[],
    },
};
