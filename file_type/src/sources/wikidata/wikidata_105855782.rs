use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855782: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_782,
        source_type: SourceType::Wikidata,
        name: "Guild Wars data",
        extensions: &["dat"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x33, 0x41, 0x4E, 0x1A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
