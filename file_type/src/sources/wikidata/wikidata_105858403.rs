use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858403: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_403,
        source_type: SourceType::Wikidata,
        name: "ERIC Project package",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x66, 0x69, 0x6C, 0x45, 0x1A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
