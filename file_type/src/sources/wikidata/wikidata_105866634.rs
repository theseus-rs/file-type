use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105866634: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_634,
        source_type: SourceType::Wikidata,
        name: "Lexi-Cross Puzzle",
        extensions: &["pzl"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x2E, 0x50, 0x5A, 0x4C])],
                },
            }],
        }],
        related_formats: &[],
    },
};
