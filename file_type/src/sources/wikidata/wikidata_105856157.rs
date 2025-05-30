use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856157: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_157,
        source_type: SourceType::Wikidata,
        name: "Twist DataBase",
        extensions: &["db"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xB1, 0x76, 0x23])],
                },
            }],
        }],
        related_formats: &[],
    },
};
