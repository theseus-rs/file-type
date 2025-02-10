use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_28207510: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_510,
        source_type: SourceType::Wikidata,
        name: "Winzle Puzzle",
        extensions: &["wzl"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4F, 0x40])],
                },
            }],
        }],
        related_formats: &[],
    },
};
