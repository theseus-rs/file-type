use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860356: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_356,
        source_type: SourceType::Wikidata,
        name: "RED files library",
        extensions: &["red"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x52, 0x52, 0x01, 0x29])],
                },
            }],
        }],
        related_formats: &[],
    },
};
