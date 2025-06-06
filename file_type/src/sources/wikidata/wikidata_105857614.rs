use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857614: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_614,
        source_type: SourceType::Wikidata,
        name: "Windmill's encrypted KIF game data archive",
        extensions: &["int"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4B, 0x49, 0x46, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
