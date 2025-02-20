use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856078: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_078,
        source_type: SourceType::Wikidata,
        name: "DataBase Diagram",
        extensions: &["dbd"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x58, 0x4D, 0x4C, 0x44, 0x69, 0x61, 0x67, 0x72, 0x61, 0x6D, 0x3E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
