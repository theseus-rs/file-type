use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850528: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_528,
        source_type: SourceType::Wikidata,
        name: "Cricket Audio Bank",
        extensions: &["ckb"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x63, 0x6B, 0x6D, 0x6B])],
                },
            }],
        }],
        related_formats: &[],
    },
};
