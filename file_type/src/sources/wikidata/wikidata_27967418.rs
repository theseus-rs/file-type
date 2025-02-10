use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_27967418: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_418,
        source_type: SourceType::Wikidata,
        name: "GYM",
        extensions: &["gym"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x47, 0x59, 0x4D, 0x58])],
                },
            }],
        }],
        related_formats: &[],
    },
};
