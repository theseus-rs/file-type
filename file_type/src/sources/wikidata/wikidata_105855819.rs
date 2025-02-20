use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855819: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_819,
        source_type: SourceType::Wikidata,
        name: "DeskMate song",
        extensions: &["sng"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x0C, 0x53, 0x4E, 0x47])],
                },
            }],
        }],
        related_formats: &[],
    },
};
