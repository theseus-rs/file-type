use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857287: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_287,
        source_type: SourceType::Wikidata,
        name: "Home Accounts account(v2)",
        extensions: &["ha2"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x48, 0x41, 0x32, 0x44])],
                },
            }],
        }],
        related_formats: &[],
    },
};
