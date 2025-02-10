use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856342: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_342,
        source_type: SourceType::Wikidata,
        name: "Skype user data",
        extensions: &["dbb"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x6C, 0x33, 0x33, 0x6C])],
                },
            }],
        }],
        related_formats: &[],
    },
};
