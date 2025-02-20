use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856947: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_947,
        source_type: SourceType::Wikidata,
        name: "Amigaguide hypertext document (var.2)",
        extensions: &["guide"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x40, 0x44, 0x41, 0x54, 0x41, 0x42, 0x41, 0x53, 0x45,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
