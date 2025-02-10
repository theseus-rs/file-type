use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856968: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_968,
        source_type: SourceType::Wikidata,
        name: "Mind Games - Renju saved game",
        extensions: &["gam"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x52, 0x65, 0x6E, 0x6A, 0x75, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
