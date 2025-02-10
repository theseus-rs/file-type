use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858803: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_803,
        source_type: SourceType::Wikidata,
        name: "bitcoin block file",
        extensions: &["dat"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xF9, 0xBE, 0xB4, 0xD9])],
                },
            }],
        }],
        related_formats: &[],
    },
};
