use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856386: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_386,
        source_type: SourceType::Wikidata,
        name: "Chromadrome 2 game data archive",
        extensions: &["dam"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x52, 0x5A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
