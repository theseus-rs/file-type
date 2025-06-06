use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856233: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_233,
        source_type: SourceType::Wikidata,
        name: "Dan Bricklin's Demo 2 demo",
        extensions: &["dbd"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x54, 0x79, 0x70, 0x65, 0x44, 0x32])],
                },
            }],
        }],
        related_formats: &[],
    },
};
