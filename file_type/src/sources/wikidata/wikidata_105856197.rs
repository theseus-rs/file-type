use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856197: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_197,
        source_type: SourceType::Wikidata,
        name: "DR Graph data",
        extensions: &["dat"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x01, 0x30, 0x0F, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
